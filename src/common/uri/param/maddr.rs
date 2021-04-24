use crate::Error;
use macros::{Display, FromIntoInner, FromStr, HasValue};
use nom::error::VerboseError;

#[derive(HasValue, Display, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct Maddr(String);

impl Maddr {
    pub fn parse(tokenizer: Tokenizer) -> Result<Self, Error> {
        use std::str::from_utf8;

        Ok(from_utf8(tokenizer.value)?.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub value: &'a [u8],
}

impl<'a> From<&'a [u8]> for Tokenizer<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self { value }
    }
}

#[allow(clippy::type_complexity)]
impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), nom::Err<VerboseError<&'a [u8]>>> {
        use nom::{
            bytes::complete::{is_not, tag},
            sequence::tuple,
        };

        let (rem, (_, maddr)) = tuple((tag("maddr="), is_not(" ;?\r\n")))(part)?;

        Ok((rem, maddr.into()))
    }
}
