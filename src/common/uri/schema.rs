#[doc(hidden)]
pub use tokenizer::Tokenizer;

/// Simple enum that holds the schema part of a URIs. This type is not a `Copy` type because
/// it can hold any `Contact` URI, like `mailto` etc.
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

impl std::fmt::Display for Schema {
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
    use super::Schema;
    use crate::{Error, NomError};
    use std::convert::TryInto;

    impl<'a> TryInto<Schema> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<Schema, Error> {
            use std::str::from_utf8;

            match from_utf8(self.value)? {
                part if part.eq_ignore_ascii_case("sip") => Ok(Schema::Sip),
                part if part.eq_ignore_ascii_case("sips") => Ok(Schema::Sips),
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
}
