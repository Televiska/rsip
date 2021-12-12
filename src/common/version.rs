use crate::Error;
#[doc(hidden)]
pub use tokenizer::Tokenizer;

use std::convert::{TryFrom, TryInto};

/// Simple enum that holds the SIP version. Defaults to `Version::V2`.
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

impl<'a> TryFrom<&'a [u8]> for Version {
    type Error = crate::Error;

    fn try_from(from: &'a [u8]) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(from)?.1.try_into()
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

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a str, char>> for Version {
    type Error = Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a str, char>) -> Result<Self, Self::Error> {
        match tokenizer.major {
            "1" => Ok(Version::V1),
            "2" => Ok(Version::V2),
            _ => Err(Self::Error::ParseError("Unrecognized SIP version".into())),
        }
    }
}

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a [u8], u8>> for Version {
    type Error = Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a [u8], u8>) -> Result<Self, Self::Error> {
        match tokenizer.major {
            b"1" => Ok(Version::V1),
            b"2" => Ok(Version::V2),
            _ => Err(Self::Error::ParseError("Unrecognized SIP version".into())),
        }
    }
}

#[doc(hidden)]
mod tokenizer {
    use crate::{AbstractInput, AbstractInputItem, GResult, GenericNomError, TokenizerError};
    use std::marker::PhantomData;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        pub major: T,
        pub minor: T,
        phantom1: PhantomData<&'a T>,
        phantom2: PhantomData<I>,
    }

    impl<'a, T, I> From<(T, T)> for Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        fn from(from: (T, T)) -> Self {
            Self {
                major: from.0,
                minor: from.1,
                phantom1: PhantomData,
                phantom2: PhantomData,
            }
        }
    }

    impl<'a, T, I> Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        pub fn tokenize(part: T) -> GResult<T, Self> {
            use nom::{bytes::complete::tag, character::complete::digit1, sequence::tuple};

            let (rem, (_, major, _, minor)) = tuple((tag("SIP/"), digit1, tag("."), digit1))(part)
                .map_err(|_: GenericNomError<'a, T>| {
                    TokenizerError::from(("version", part)).into()
                })?;

            Ok((rem, (major, minor).into()))
        }
    }
}

#[cfg(feature = "test-utils")]
impl testing_utils::Randomize for Version {
    fn random() -> Self {
        //is anyone using V1 ?!
        Self::V2
    }
}
