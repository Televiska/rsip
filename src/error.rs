#![allow(dead_code)]

use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    MissingHeader(Header),
    InvalidParam(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingHeader(header) => write!(f, "Libsip error: missing header: {:?}", header),
            Self::InvalidParam(inner) => write!(f, "Libsip error: invalid header param: {}", inner),
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
