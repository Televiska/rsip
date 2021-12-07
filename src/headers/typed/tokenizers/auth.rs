use crate::{
    headers::{auth, typed::Tokenize},
    Error,
};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct AuthTokenizer<'a> {
    pub scheme: auth::scheme::Tokenizer<'a, &'a str, char>,
    pub params: Vec<(&'a str, &'a str)>,
}

impl<'a> Tokenize<'a> for AuthTokenizer<'a> {
    fn tokenize(part: &'a str) -> Result<Self, Error> {
        use crate::parser_utils::is_empty_or_fail_with;
        use crate::NomStrError;
        use nom::{
            branch::alt,
            bytes::complete::{tag, take_until},
            character::complete::space0,
            combinator::{map, opt, rest},
            multi::many1,
            sequence::{delimited, tuple},
        };

        let params = map(
            tuple((
                space0,
                take_until("="),
                tag("="),
                alt((
                    delimited(tag("\""), take_until("\""), tag("\"")),
                    take_until(","),
                    take_until(" "),
                    rest,
                )),
                opt(tag(",")),
            )),
            |tuple| (tuple.1, tuple.3),
        );

        let (rem, _) =
            space0(part).map_err(|_: NomStrError<'a>| Error::tokenizer(("auth header", part)))?;
        let (rem, scheme) = auth::scheme::Tokenizer::tokenize(rem)
            .map_err(|_| Error::tokenizer(("auth header scheme", part)))?;
        let (rem, params) = many1(params)(rem)
            .map_err(|_: NomStrError<'a>| Error::tokenizer(("auth header params", part)))?;
        is_empty_or_fail_with(rem, ("auth header params", part))?;

        Ok(Self { scheme, params })
    }
}
