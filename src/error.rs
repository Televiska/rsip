#![allow(dead_code)]

use nom::error::VerboseError;
use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    MissingHeader(Header),
    InvalidParam(String),
    //TODO: needs fixing
    ParseError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingHeader(header) => write!(f, "rsip error: missing header: {:?}", header),
            Self::InvalidParam(inner) => write!(f, "rsip error: invalid header param: {}", inner),
            Self::ParseError(inner) => write!(
                f,
                "rsip error: could not parse header through libsip: {}",
                inner
            ),
        }
    }
}

impl StdError for Error {}

#[derive(Debug)]
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
}

impl From<nom::Err<VerboseError<&[u8]>>> for Error {
    fn from(error: nom::Err<VerboseError<&[u8]>>) -> Self {
        Self::ParseError(error.to_string())
    }
}
