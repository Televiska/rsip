use crate::{headers::typed::Tokenize, Error};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct NameParamsTokenizer<'a> {
    pub name: &'a str,
    pub params: Vec<(&'a str, &'a str)>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct NameParamsListTokenizer<'a>(pub Vec<NameParamsTokenizer<'a>>);

impl<'a> Tokenize<'a> for NameParamsTokenizer<'a> {
    fn tokenize(part: &'a str) -> Result<Self, Error> {
        use crate::parser_utils::is_empty_or_fail_with;
        use crate::NomStrError;
        use nom::{
            branch::alt,
            bytes::complete::{tag, take_until},
            character::complete::space0,
            combinator::{map, opt, rest},
            multi::many0,
            sequence::{delimited, tuple},
        };

        let params = map(
            tuple((
                space0,
                tag(";"),
                space0,
                take_until("="),
                tag("="),
                alt((
                    delimited(tag("\""), take_until("\""), tag("\"")),
                    take_until(";"),
                    rest,
                )),
                opt(tag(",")),
            )),
            |tuple| (tuple.3, tuple.5),
        );

        let (rem, name) = alt((take_until(";"), rest))(part)
            .map_err(|_: NomStrError<'a>| Error::tokenizer(("media type", part)))?;
        let (rem, params) = many0(params)(rem)
            .map_err(|_: NomStrError<'a>| Error::tokenizer(("media type params", part)))?;
        is_empty_or_fail_with(rem, ("media type", rem))?;

        Ok(Self { name, params })
    }
}

impl<'a> Tokenize<'a> for NameParamsListTokenizer<'a> {
    fn tokenize(part: &'a str) -> Result<Self, Error> {
        use crate::NomStrError;
        use nom::{
            bytes::complete::{tag, take_until},
            character::complete::space0,
            multi::many0,
            sequence::{terminated, tuple},
        };

        let stopbreak = terminated(take_until(","), tuple((tag(","), space0)));

        let (rem, media_types) = many0(stopbreak)(part)
            .map_err(|_: NomStrError<'a>| Error::tokenizer(("list media type params", part)))?;
        let mut media_types = media_types
            .into_iter()
            .map(NameParamsTokenizer::tokenize)
            .collect::<Result<Vec<NameParamsTokenizer>, Error>>()?;
        if !rem.is_empty() {
            media_types.push(NameParamsTokenizer::tokenize(rem)?)
        }

        Ok(Self(media_types))
    }
}
