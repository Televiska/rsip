mod tokenizer_error;

pub use tokenizer_error::TokenizerError;

use std::{error::Error as StdError, fmt};

/// The `Error` enum indicates that something went wrong
///
/// It has 6 variants:
///
/// * `MissingHeader` that a header that is expected to be found was completely missing.
/// There are some headers that are required everywhere in SIP, like `From`, `To` etc.
/// * `InvalidParam` means some header parser did not succeed, and the reason for it is
/// a missing or invalid parameter. Inner `String` should have the nom verbose error about it.
/// * `ParseError` indicates a general parsing error. Inner `String` should have the verbose nom error
/// giving hints on what went wrong.
/// * `TokenizeError` indicates a general tokenizer error. Inner `String` should have the verbose nom error
/// giving hints on what went wrong. A Tokenizer just tries to break the `bytes` into
/// parts/components of rsip structs, but never tries to parse/convert to a specific type.
/// So if a tokenizer fails and you get this error, it means it couldn't even manage to break the
/// SIP message in the correct breakpoints.
/// * `Utf8Error` indicates that the `from_utf8` std method completely failed. At least you should
/// get the information regarding which header had this issue.
/// * `Unexpected` indicates any other error.
///
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Error {
    MissingHeader(String),
    InvalidParam(String),
    ParseError(String),
    TokenizeError(String),
    Utf8Error(String),
    Unexpected(String),
}

impl Error {
    pub fn tokenizer<'a, S, T>(tuple: (S, T)) -> Self
    where
        S: Into<String>,
        T: Into<&'a bstr::BStr>,
    {
        Self::TokenizeError(format!(
            "failed to tokenize {}: {}",
            tuple.0.into(),
            tuple.1.into()
        ))
    }

    pub fn missing_header(header: &'static str) -> Self {
        Self::MissingHeader(header.into())
    }
}

impl From<TokenizerError> for Error {
    fn from(from: TokenizerError) -> Self {
        Self::TokenizeError(from.context)
    }
}

impl From<nom::Err<TokenizerError>> for Error {
    fn from(from: nom::Err<TokenizerError>) -> Self {
        match from {
            nom::Err::Incomplete(_) => Self::Unexpected("incomplete parsing?".into()),
            nom::Err::Error(e) => e.into(),
            nom::Err::Failure(e) => e.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingHeader(inner) => write!(f, "rsip error: missing header: {}", inner),
            Self::InvalidParam(inner) => write!(f, "rsip error: invalid header param: {}", inner),
            Self::ParseError(inner) => write!(f, "rsip error: could not parse part: {}", inner),
            Self::TokenizeError(inner) => write!(f, "Tokenizer error: {}", inner),
            Self::Unexpected(inner) => write!(f, "rsip quite unexpected error: {}", inner),
            Self::Utf8Error(inner) => write!(f, "rsip error: invalid utf8 ({})", inner),
        }
    }
}

impl StdError for Error {}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Self::Utf8Error(error.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::ParseError(error.to_string())
    }
}
