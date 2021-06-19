#![allow(dead_code)]

use nom::error::VerboseError;
use std::{error::Error as StdError, fmt};

//TODO: add tokenizer error for nom errors and use parse errors for u8 -> type errors
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Error {
    MissingHeader(Header),
    InvalidParam(String),
    //TODO: needs fixing
    ParseError(String),
    Utf8Error(Header, String),
    Unexpected(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingHeader(header) => write!(f, "rsip error: missing header: {:?}", header),
            Self::InvalidParam(inner) => write!(f, "rsip error: invalid header param: {}", inner),
            Self::ParseError(inner) => write!(f, "rsip error: could not parse part: {}", inner),
            Self::Unexpected(inner) => write!(f, "rsip quite unexpected error: {}", inner),
            Self::Utf8Error(header, reason) => write!(
                f,
                "rsip error: could not parse header {:?}: invalid utf8 ({})",
                header, reason
            ),
        }
    }
}

impl StdError for Error {}

//TODO: move out of here
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Header {
    To,
    Contact,
    From,
    ReplyTo,
    CSeq,
    MaxForwards,
    Event,
    Expires,
    Accept,
    ContentLength,
    Allow,
    UserAgent,
    CallId,
    ContentType,
    ContentLanguage,
    ContentEncoding,
    AcceptLanguage,
    AcceptEncoding,
    AlertInfo,
    ErrorInfo,
    AuthenticationInfo,
    Authorization,
    CallInfo,
    InReplyTo,
    ContentDisposition,
    Date,
    MinExpires,
    MimeVersion,
    Organization,
    ProxyAuthenticate,
    ProxyAuthorization,
    ProxyRequire,
    Require,
    RetryAfter,
    Route,
    Subject,
    SubscriptionState,
    RecordRoute,
    Server,
    Supported,
    Timestamp,
    Unsupported,
    Warning,
    Via,
    Priority,
    WwwAuthenticate,
    XFsSendingMessage,
    //TODO: this is not a header, need a fix here
    Status,
}

impl From<nom::Err<VerboseError<&[u8]>>> for Error {
    fn from(error: nom::Err<VerboseError<&[u8]>>) -> Self {
        use std::str::from_utf8;

        let transform_errors = |error: VerboseError<&[u8]>| match error
            .errors
            .iter()
            .map(|error_item| {
                from_utf8(error_item.0)
                    .map(|e| format!("{:?}: {}", error_item.1, String::from(e)))
                    .map_err(Error::from)
            })
            .collect::<Result<Vec<String>, Self>>()
        {
            Ok(vec) => Error::ParseError(vec.join(", ")),
            Err(err) => err,
        };

        match error {
            nom::Err::Failure(e) => transform_errors(e),
            nom::Err::Error(e) => transform_errors(e),
            _ => Error::Unexpected(error.to_string()),
        }
    }
}

impl From<nom::Err<VerboseError<&str>>> for Error {
    fn from(error: nom::Err<VerboseError<&str>>) -> Self {
        let transform_errors = |error: VerboseError<&str>| {
            error
                .errors
                .iter()
                .map(|error_item| format!("{:?}: {}", error_item.1, error_item.0))
                .collect::<Vec<String>>()
                .join(", ")
        };

        match error {
            nom::Err::Failure(e) => Error::ParseError(transform_errors(e)),
            nom::Err::Error(e) => Error::ParseError(transform_errors(e)),
            _ => Error::Unexpected(error.to_string()),
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Self::Utf8Error(Header::Status, error.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::ParseError(error.to_string())
    }
}
