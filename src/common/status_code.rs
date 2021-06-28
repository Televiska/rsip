pub use tokenizer::Tokenizer;

macro_rules! foo {
    ($($name:ident => $code:expr),*) => {
        #[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
        pub enum StatusCode {
            $(
                $name,
            )*
            Other(u16, String),
        }

        impl StatusCode {
            pub fn code(&self) -> u16 {
                match self {
                    $(
                        Self::$name => $code,
                    )*
                    Self::Other(code, _) => *code,
                }
            }
        }

        impl From<u16> for StatusCode {
            fn from(code: u16) -> Self {
                match code {
                    $(
                        $code => Self::$name,
                    )*
                    code => Self::Other(code, "Other".into()),
                }

            }
        }

        impl std::fmt::Display for StatusCode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$name => write!(f, "{} {}", stringify!($code), stringify!($name)),
                    )*
                    Self::Other(code, reason) => write!(f, "{} {}", code, reason),
                }
            }
        }

        fn match_from<'a>(code: u16, reason: &'a [u8]) -> Result<StatusCode, crate::Error> {
            use std::str::from_utf8;

            match (code, reason) {
                $(
                    ($code, _) => Ok(StatusCode::$name),
                )*
                (code, reason) => Ok(StatusCode::Other(code, from_utf8(reason)?.into())),
            }
        }
    }
}

foo!(Trying => 100,
    Ringing => 180,
    CallIsBeingForwarded => 181,
    Queued => 182,
    SessionProgress => 183,
    EarlyDialogTerminated => 199,
    OK => 200,
    Accepted => 201,
    NoNotification => 204,
    MultipleChoices => 300,
    MovedPermanently => 301,
    MovedTemporarily => 302,
    UseProxy => 305,
    AlternativeService => 380,
    BadRequest => 400,
    Unauthorized => 401,
    PaymentRequired => 402,
    Forbidden => 403,
    NotFound => 404,
    MethodNotAllowed => 405,
    NotAcceptable => 406,
    ProxyAuthenticationRequired => 407,
    RequestTimeout => 408,
    Conflict => 409,
    Gone => 410,
    LengthRequired => 411,
    ConditionalRequestFailed => 412,
    RequestEntityTooLarge => 413,
    RequestUriTooLong => 414,
    UnsupportedMediaType => 415,
    UnsupportedUriScheme => 416,
    UnknownResourcePriority => 417,
    BadExtension => 420,
    ExtensionRequired => 421,
    SessionIntervalTooSmall => 422,
    IntervalTooBrief => 423,
    BadLocationInformation => 424,
    UseIdentityHeader => 428,
    ProvideReferrerIdentity => 429,
    AnonymityDisallowed => 433,
    BadIdentityInfo => 436,
    UnsupportedCertificate => 437,
    InvalidIdentityHeader => 438,
    FirstHopLacksOutboundSupport => 439,
    MaxBreadthExceeded => 440,
    BadInfoPackage => 469,
    ConsentNeeded => 470,
    TemporarilyUnavailable => 480,
    CallTransactionDoesNotExist => 481,
    LoopDetected => 482,
    TooManyHops => 483,
    AddressIncomplete => 484,
    Ambiguous => 485,
    BusyHere => 486,
    RequestTerminated => 487,
    NotAcceptableHere => 488,
    BadEvent => 489,
    RequestPending => 491,
    Undecipherable => 493,
    SecurityAgreementRequired => 494,
    ServerInternalError => 500,
    NotImplemented => 501,
    BadGateway => 502,
    ServiceUnavailable => 503,
    ServerTimeOut => 504,
    VersionNotSupported => 505,
    MessageTooLarge => 513,
    PreconditionFailure => 580,
    BusyEverywhere => 600,
    Decline => 603,
    DoesNotExistAnywhere => 604,
    NotAcceptableGlobal => 606,
    Unwanted => 607
);

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
pub enum StatusCodeKind {
    Provisional,
    Successful,
    Redirection,
    RequestFailure,
    ServerFailure,
    GlobalFailure,
    Other,
}

impl StatusCode {
    pub fn kind(&self) -> StatusCodeKind {
        let code = self.code();
        match code {
            code if (100..200).contains(&code) => StatusCodeKind::Provisional,
            code if (200..300).contains(&code) => StatusCodeKind::Successful,
            code if (300..400).contains(&code) => StatusCodeKind::Redirection,
            code if (400..500).contains(&code) => StatusCodeKind::RequestFailure,
            code if (500..600).contains(&code) => StatusCodeKind::ServerFailure,
            code if (600..700).contains(&code) => StatusCodeKind::GlobalFailure,
            _ => StatusCodeKind::Other,
        }
    }
}

impl From<StatusCode> for u16 {
    fn from(from: StatusCode) -> u16 {
        from.code()
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        Self::OK
    }
}

//Here we decide to completely ignore the reason if the code can be mapped to a well known status
pub mod tokenizer {
    use super::StatusCode;
    use crate::{Error, NomError};
    use std::convert::TryInto;

    impl<'a> TryInto<StatusCode> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<StatusCode, Error> {
            use super::match_from;
            use std::str::from_utf8;

            match_from(from_utf8(self.code)?.parse::<u16>()?, self.reason)
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Tokenizer<'a> {
        pub code: &'a [u8],
        pub reason: &'a [u8],
    }

    impl<'a> From<(&'a [u8], &'a [u8])> for Tokenizer<'a> {
        fn from(tuple: (&'a [u8], &'a [u8])) -> Self {
            Self {
                code: tuple.0,
                reason: tuple.1,
            }
        }
    }

    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
            use nom::{
                bytes::complete::{tag, take_until},
                character::complete::digit1,
                sequence::tuple,
            };

            let (rem, (code, _, reason, _)) =
                tuple((digit1, tag(" "), take_until("\r\n"), tag("\r\n")))(part)?;

            Ok((rem, Self { code, reason }))
        }
    }
}
