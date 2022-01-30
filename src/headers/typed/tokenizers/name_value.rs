use crate::{headers::typed::Tokenize, Error};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct NameValueTokenizer<'a> {
    pub params: Vec<(&'a str, &'a str)>,
}

impl<'a> From<Vec<(&'a str, &'a str)>> for NameValueTokenizer<'a> {
    fn from(params: Vec<(&'a str, &'a str)>) -> Self {
        Self { params }
    }
}

impl<'a> Tokenize<'a> for NameValueTokenizer<'a> {
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

        let (rem, params) = many1(params)(part)
            .map_err(|_: NomStrError<'a>| Error::tokenizer(("name value params", part)))?;
        is_empty_or_fail_with(rem, ("name value params", part))?;

        Ok(Self { params })
    }
}
