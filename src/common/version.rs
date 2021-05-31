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
            match self.major {
                b"1" => Ok(Version::V1),
                b"2" => Ok(Version::V2),
                _ => Err(Error::ParseError("Unrecognized SIP version".into())),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Tokenizer<'a> {
        pub major: &'a [u8],
        pub minor: &'a [u8],
    }

    impl<'a> From<(&'a [u8], &'a [u8])> for Tokenizer<'a> {
        fn from(tuple: (&'a [u8], &'a [u8])) -> Self {
            Self {
                major: tuple.0,
                minor: tuple.1,
            }
        }
    }

    impl<'a> From<&'a [u8]> for Tokenizer<'a> {
        fn from(major: &'a [u8]) -> Self {
            Self { major, minor: b"0" }
        }
    }

    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
            use nom::{bytes::complete::tag, character::complete::digit1, sequence::tuple};

            let (rem, (_, major, _, minor)) = tuple((tag("SIP/"), digit1, tag("."), digit1))(part)?;

            Ok((rem, (major, minor).into()))
        }
    }
}
