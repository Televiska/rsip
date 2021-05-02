use crate::{Error, NomError};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Version {
    V1,
    V2,
    //Custom(String)
}

impl Default for Version {
    fn default() -> Self {
        Self::V2
    }
}

impl Version {
    pub fn parse(tokenizer: Tokenizer) -> Result<Self, Error> {
        use nom::{bytes::complete::tag, character::complete::digit1, sequence::tuple};
        let (_, (_, major, _, _)) =
            tuple((tag("SIP/"), digit1, tag("."), digit1))(tokenizer.value)?;

        match major {
            b"1" => Ok(Self::V1),
            b"2" => Ok(Self::V2),
            _ => Err(Error::ParseError("Unrecognized SIP version".into())),
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

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
        use nom::{
            branch::alt,
            bytes::complete::{tag, take_till},
        };

        let (rem, version) = take_till(|c| c == b' ' || c == b'\r')(part)?;
        let (rem, _) = alt((tag(" "), tag("\r\n")))(rem)?;

        Ok((rem, version.into()))
    }
}
