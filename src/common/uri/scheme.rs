#[doc(hidden)]
pub use tokenizer::Tokenizer;

use crate::Error;

/// Simple enum that holds the scheme part of a URIs. This type is not a `Copy` type because
/// it can hold any `Contact` URI, like `mailto` etc.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Scheme {
    Sip,
    Sips,
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

    pub fn is_sip_secure(&self) -> Result<bool, Error> {
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
            Self::Other(inner) => write!(f, "{}", inner),
        }
    }
}

#[doc(hidden)]
pub mod tokenizer {
    use super::Scheme;
    use crate::{Error, IResult, TokenizerError};
    use std::convert::TryInto;

    impl<'a> TryInto<Scheme> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<Scheme, Error> {
            use std::str::from_utf8;

            match from_utf8(self.value)? {
                part if part.eq_ignore_ascii_case("sip") => Ok(Scheme::Sip),
                part if part.eq_ignore_ascii_case("sips") => Ok(Scheme::Sips),
                part => Ok(Scheme::Other(part.into())),
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
        pub fn tokenize(part: &'a [u8]) -> IResult<Self> {
            use crate::NomError;
            use nom::{
                branch::alt,
                bytes::complete::{tag, tag_no_case, take_until},
                sequence::tuple,
            };

            let (rem, (scheme, _)) = alt((
                tuple((tag_no_case("sip"), tag(":"))),
                tuple((tag_no_case("sips"), tag(":"))),
                tuple((take_until("://"), tag("://"))),
            ))(part)
            .map_err(|_: NomError<'a>| TokenizerError::from(("scheme", part)).into())?;

            Ok((rem, scheme.into()))
        }
    }
}

#[cfg(feature = "test-utils")]
impl testing_utils::Randomize for Scheme {
    fn random() -> Self {
        testing_utils::sample(&[
            Scheme::Sip,
            Scheme::Sips,
            Scheme::Other(testing_utils::rand_str_of(3)),
        ])
    }
}
