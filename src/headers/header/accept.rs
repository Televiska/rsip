use crate::{headers::Header, Error};
use macros::{Display, FromIntoInner, FromStr, HasValue, IntoHeader};

#[derive(HasValue, Display, IntoHeader, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct Accept(String);

impl Accept {
    pub fn parse<'a>(tokenizer: Tokenizer<'a>) -> Result<Self, Error> {
        use std::str::from_utf8;

        Ok(Self(from_utf8(tokenizer.value)?.into()))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tokenizer<'a> {
    pub value: &'a [u8],
}

impl<'a> From<&'a [u8]> for Tokenizer<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self { value }
    }
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), Error> {
        use nom::{
            bytes::complete::{tag, tag_no_case, take_until},
            character::complete::space0,
            sequence::delimited,
            sequence::tuple,
        };

        let (rem, value) = delimited(
            tuple((tag_no_case("accept:"), space0)),
            take_until("\r\n"),
            tag("\r\n"),
        )(part)?;

        Ok((rem, value.into()))
    }
}
