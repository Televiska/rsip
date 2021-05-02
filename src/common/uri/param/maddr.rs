use crate::{Error, NomError};
use macros::{Display, FromIntoInner, FromStr, HasValue};
use std::convert::TryFrom;

#[derive(HasValue, Display, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct Maddr(String);

impl Maddr {
    pub fn parse(tokenizer: Tokenizer) -> Result<Self, Error> {
        use std::str::from_utf8;

        Ok(from_utf8(tokenizer.value)?.into())
    }
}

impl<'a> TryFrom<Tokenizer<'a>> for Maddr {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Error> {
        Self::parse(tokenizer)
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
    pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
        use nom::{
            bytes::complete::{is_not, tag},
            sequence::tuple,
        };

        let (rem, (_, maddr)) = tuple((tag("maddr="), is_not(" ;?\r\n")))(part)?;

        Ok((rem, maddr.into()))
    }
}
