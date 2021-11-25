#[doc(hidden)]
pub use tokenizer::Tokenizer;

/// The `Scheme`, as part of the SIP Authorization framework, found in headers like
/// [Authorization](super::super::typed::Authorization) and
/// [WwwAuthenticate](super::super::typed::WwwAuthenticate)
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Scheme {
    Digest,
    Other(String),
}

impl Default for Scheme {
    fn default() -> Self {
        Self::Digest
    }
}

impl std::fmt::Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Digest => write!(f, "Digest"),
            Self::Other(inner) => write!(f, "{}", inner),
        }
    }
}

impl<'a> std::convert::TryFrom<Tokenizer<'a, &'a [u8], u8>> for Scheme {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer<'a, &'a [u8], u8>) -> Result<Self, Self::Error> {
        use std::str::from_utf8;

        match from_utf8(tokenizer.value)? {
            part if part.eq_ignore_ascii_case("digest") => Ok(Self::Digest),
            part => Ok(Self::Other(part.into())),
        }
    }
}

impl<'a> std::convert::TryFrom<Tokenizer<'a, &'a str, char>> for Scheme {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer<'a, &'a str, char>) -> Result<Self, Self::Error> {
        match tokenizer.value {
            part if part.eq_ignore_ascii_case("digest") => Ok(Self::Digest),
            part => Ok(Self::Other(part.into())),
        }
    }
}

#[doc(hidden)]
mod tokenizer {
    use crate::{AbstractInput, GResult, GenericNomError, TokenizerError};
    use std::marker::PhantomData;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: nom::AsChar,
    {
        pub value: T,
        phantom1: PhantomData<&'a T>,
        phantom2: PhantomData<I>,
    }

    impl<'a, T, I> From<T> for Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: nom::AsChar,
    {
        fn from(value: T) -> Self {
            Self {
                value,
                phantom1: PhantomData,
                phantom2: PhantomData,
            }
        }
    }

    impl<'a, T, I> Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: nom::AsChar,
    {
        pub fn tokenize(part: T) -> GResult<T, Self> {
            use nom::{branch::alt, bytes::complete::take_until, combinator::rest};

            let (rem, scheme) =
                alt((take_until(" "), rest))(part).map_err(|_: GenericNomError<'a, T>| {
                    TokenizerError::from(("scheme (header)", part)).into()
                })?;

            Ok((rem, Self::from(scheme)))
        }
    }
}
