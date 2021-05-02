use crate::{Error, NomError};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Schema {
    Sip,
    Sips,
    Other(String),
}

impl Default for Schema {
    fn default() -> Self {
        Self::Sip
    }
}

impl Schema {
    pub fn parse(tokenizer: Tokenizer) -> Result<Self, Error> {
        use std::str::from_utf8;

        match from_utf8(tokenizer.value)? {
            part if part.eq_ignore_ascii_case("sip") => Ok(Self::Sip),
            part if part.eq_ignore_ascii_case("sips") => Ok(Self::Sips),
            part => Err(Error::ParseError(format!("Invalid method `{}`", part))),
        }
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
            branch::alt,
            bytes::complete::{tag, tag_no_case, take_until},
            sequence::tuple,
        };

        let (rem, (schema, _)) = alt((
            tuple((tag_no_case("sip"), tag(":"))),
            tuple((tag_no_case("sips"), tag(":"))),
            tuple((take_until("://"), tag("://"))),
        ))(part)?;

        Ok((rem, schema.into()))
    }
}
