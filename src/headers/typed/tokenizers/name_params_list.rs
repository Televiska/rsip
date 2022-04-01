use crate::{headers::typed::Tokenize, Error};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct NameParamsListTokenizer<'a>(pub Vec<super::NameParamsTokenizer<'a>>);

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
            .map(super::NameParamsTokenizer::tokenize)
            .collect::<Result<Vec<super::NameParamsTokenizer>, Error>>()?;
        if !rem.is_empty() {
            media_types.push(super::NameParamsTokenizer::tokenize(rem)?)
        }

        Ok(Self(media_types))
    }
}
