#[doc(hidden)]
pub use tokenizer::Tokenizer;

use crate::Error;

/// Simple enum that holds the scheme part of a URIs. This type is not a `Copy` type because
/// it can hold any `Contact` URI, like `mailto` etc.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Scheme {
    Sip,
    Sips,
    // A tel scheme from RFC 2806.
    Tel,
    Other(String),
}

impl Scheme {
    pub fn default_transport(&self) -> Result<crate::Transport, Error> {
        use crate::Transport;

        match self {
            Scheme::Sip => Ok(Transport::Udp),
            Scheme::Sips => Ok(Transport::Tls),
            _ => Err(Error::Unexpected("unsupported scheme".into())),
        }
    }

    pub fn is_sips(&self) -> Result<bool, Error> {
        match self {
            Self::Sip => Ok(false),
            Self::Sips => Ok(true),
            _ => Err(Error::Unexpected("Not sip scheme".into())),
        }
    }
}

impl Default for Scheme {
    fn default() -> Self {
        Self::Sip
    }
}

impl std::fmt::Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sip => write!(f, "sip"),
            Self::Sips => write!(f, "sips"),
            Self::Tel => write!(f, "tel"),
            Self::Other(inner) => write!(f, "{}", inner),
        }
    }
}

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a str, char>> for Scheme {
    type Error = Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a str, char>) -> Result<Self, Self::Error> {
        match tokenizer.value {
            part if part.eq_ignore_ascii_case("sip") => Ok(Scheme::Sip),
            part if part.eq_ignore_ascii_case("sips") => Ok(Scheme::Sips),
            part if part.eq_ignore_ascii_case("tel") => Ok(Scheme::Tel),
            part => Ok(Scheme::Other(part.into())),
        }
    }
}

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a [u8], u8>> for Scheme {
    type Error = Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a [u8], u8>) -> Result<Self, Self::Error> {
        use std::str::from_utf8;

        Self::try_from(Tokenizer::from(from_utf8(tokenizer.value)?))
    }
}

#[doc(hidden)]
pub mod tokenizer {
    use crate::{AbstractInput, AbstractInputItem, GResult, GenericNomError, TokenizerError};
    use std::marker::PhantomData;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        pub value: T,
        phantom1: PhantomData<&'a T>,
        phantom2: PhantomData<I>,
    }

    impl<'a, T, I> From<T> for Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        fn from(value: T) -> Self {
            Self {
                value,
                phantom1: Default::default(),
                phantom2: Default::default(),
            }
        }
    }

    impl<'a, T, I> Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        pub fn tokenize(part: T) -> GResult<T, Self> {
            use nom::{
                branch::alt,
                bytes::complete::{tag, tag_no_case, take_until},
                sequence::tuple,
            };

            let (rem, (scheme, _)) = alt((
                tuple((tag_no_case("sip"), tag(":"))),
                tuple((tag_no_case("sips"), tag(":"))),
                tuple((tag_no_case("tel"), tag(":"))),
                tuple((take_until("://"), tag("://"))),
            ))(part)
            .map_err(|_: GenericNomError<'a, T>| TokenizerError::from(("scheme", part)).into())?;

            Ok((rem, Tokenizer::from(scheme)))
        }
    }
}

#[cfg(feature = "test-utils")]
impl testing_utils::Randomize for Scheme {
    fn random() -> Self {
        use testing_utils::sample;
        sample(&[
            Scheme::Sip,
            Scheme::Sips,
            Scheme::Other(sample(&["http", "https", &testing_utils::rand_str_of(3)]).into()),
        ])
    }
}
