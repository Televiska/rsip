use std::{error::Error as StdError, fmt};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TokenizerError {
    pub context: String,
}

impl<'a, S, T> From<(S, T)> for TokenizerError
where
    S: Into<String>,
    T: Into<&'a bstr::BStr>,
{
    fn from(from: (S, T)) -> Self {
        Self {
            context: format!("failed to tokenize {}: {}", from.0.into(), from.1.into()),
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

impl<'a, T: Into<&'a bstr::BStr>> nom::error::ParseError<T> for TokenizerError {
    fn from_error_kind(input: T, kind: nom::error::ErrorKind) -> Self {
        Self {
            context: format!("could not tokenize ({:?}): {}", kind, input.into()),
        }
    }
    fn append(input: T, kind: nom::error::ErrorKind, other: Self) -> Self {
        Self {
            context: format!(
                "{}. Also, could not tokenize ({:?}): {}",
                other,
                kind,
                input.into(),
            ),
        }
    }

    fn from_char(input: T, c: char) -> Self {
        Self {
            context: format!("was expecting char {} in: {}", c, input.into()),
        }
    }

    fn or(self, other: Self) -> Self {
        Self {
            context: format!("tokenizer error: {} or {}", self, other.context),
        }
    }
}
