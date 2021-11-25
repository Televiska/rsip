use bstr::ByteSlice;
use std::{error::Error as StdError, fmt};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TokenizerError {
    pub context: String,
}

impl<'a, T> From<(&'static str, T)> for TokenizerError
where
    T: Into<&'a bstr::BStr>,
{
    fn from(from: (&'static str, T)) -> Self {
        Self {
            context: format!("failed to tokenize {}: {}", from.0, from.1.into()),
        }
    }
}

impl From<&'static str> for TokenizerError {
    fn from(from: &'static str) -> Self {
        Self {
            context: from.into(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<nom::Err<Self>> for TokenizerError {
    fn into(self) -> nom::Err<Self> {
        nom::Err::Error(self)
    }
}

#[allow(clippy::from_over_into)]
impl<'a, T> Into<crate::IResult<'a, T>> for TokenizerError {
    fn into(self) -> crate::IResult<'a, T> {
        Err(nom::Err::Error(self))
    }
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tokenizer error: {}", self.context)
    }
}

impl StdError for TokenizerError {}

impl nom::error::ParseError<&[u8]> for TokenizerError {
    fn from_error_kind(input: &[u8], kind: nom::error::ErrorKind) -> Self {
        Self {
            context: format!("could not tokenize: {} ({:?})", input.as_bstr(), kind),
        }
    }
    fn append(input: &[u8], kind: nom::error::ErrorKind, other: Self) -> Self {
        Self {
            context: format!(
                "{}. Also, could not tokenize: {} ({:?})",
                other,
                input.as_bstr(),
                kind
            ),
        }
    }

    fn from_char(input: &[u8], c: char) -> Self {
        Self {
            context: format!("was expecting char {} in: {}", c, input.as_bstr()),
        }
    }

    fn or(self, other: Self) -> Self {
        Self {
            context: format!("tokenizer error: {} or {}", self, other.context),
        }
    }
}

impl nom::error::ParseError<&str> for TokenizerError {
    fn from_error_kind(input: &str, kind: nom::error::ErrorKind) -> Self {
        Self {
            context: format!("could not tokenize: {} ({:?})", input, kind),
        }
    }
    fn append(input: &str, kind: nom::error::ErrorKind, other: Self) -> Self {
        Self {
            context: format!(
                "{}. Also, could not tokenize: {} ({:?})",
                other, input, kind
            ),
        }
    }

    fn from_char(input: &str, c: char) -> Self {
        Self {
            context: format!("was expecting char {} in: {}", c, input),
        }
    }

    fn or(self, other: Self) -> Self {
        Self {
            context: format!("tokenizer error: {} or {}", self, other.context),
        }
    }
}
