pub use tokenizer::Tokenizer;

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

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V1 => write!(f, "SIP/1.0"),
            Self::V2 => write!(f, "SIP/2.0"),
        }
    }
}

mod tokenizer {
    use super::Version;
    use crate::{Error, NomError};
    use std::convert::TryInto;

    impl<'a> TryInto<Version> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<Version, Error> {
            use nom::{bytes::complete::tag, character::complete::digit1, sequence::tuple};
            let (_, (_, major, _, _)) = tuple((tag("SIP/"), digit1, tag("."), digit1))(self.value)?;

            match major {
                b"1" => Ok(Version::V1),
                b"2" => Ok(Version::V2),
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
}
