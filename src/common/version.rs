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

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a [u8]>> for Version {
    type Error = crate::Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a [u8]>) -> Result<Self, Self::Error> {
        match tokenizer.major {
            b"1" => Ok(Version::V1),
            b"2" => Ok(Version::V2),
            _ => Err(Self::Error::ParseError("Unrecognized SIP version".into())),
        }
    }
}

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a str>> for Version {
    type Error = crate::Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a str>) -> Result<Self, Self::Error> {
        match tokenizer.major {
            "1" => Ok(Version::V1),
            "2" => Ok(Version::V2),
            _ => Err(Self::Error::ParseError("Unrecognized SIP version".into())),
        }
    }
}

mod tokenizer {
    use crate::AbstractInput;
    use crate::GenericNomError;
    use std::marker::PhantomData;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Tokenizer<'a, T>
    where
        T: AbstractInput<'a>,
    {
        pub major: T,
        pub minor: T,
        phantom: PhantomData<&'a T>,
    }

    impl<'a, T> From<(T, T)> for Tokenizer<'a, T>
    where
        T: AbstractInput<'a>,
    {
        fn from(from: (T, T)) -> Self {
            Self {
                major: from.0,
                minor: from.1,
                phantom: PhantomData,
            }
        }
    }

    impl<'a, T> Tokenizer<'a, T>
    where
        T: AbstractInput<'a>,
    {
        pub fn tokenize(part: T) -> Result<(T, Self), GenericNomError<'a, T>> {
            use nom::{
                bytes::complete::{tag, take_until},
                sequence::tuple,
            };

            let (rem, (_, major, _, minor)) =
                tuple((tag("SIP/"), take_until("."), tag("."), tag("0")))(part)?;

            Ok((rem, (major, minor).into()))
        }
    }
}
