pub use tokenizer::Tokenizer;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub enum StatusCode {
    Trying,
    Ringing,
    CallIsBeingForwarded,
    Queued,
    SessionProgress,
    EarlyDialogTerminated,
    Ok,
    Accepted,
    NoNotification,
    MultipleChoices,
    MovedPermanently,
    MovedTemporarily,
    UseProxy,
    AlternativeService,
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    ConditionalRequestFailed,
    RequestEntityTooLarge,
    RequestUriTooLong,
    UnsupportedMediaType,
    UnsupportedUriScheme,
    UnknownResourcePriority,
    BadExtension,
    ExtensionRequired,
    SessionIntervalTooSmall,
    IntervalTooBrief,
    BadLocationInformation,
    UseIdentityHeader,
    ProvideReferrerIdentity,
    AnonymityDisallowed,
    BadIdentityInfo,
    UnsupportedCertificate,
    InvalidIdentityHeader,
    FirstHopLacksOutboundSupport,
    MaxBreadthExceeded,
    BadInfoPackage,
    ConsentNeeded,
    TemporarilyUnavailable,
    CallTransactionDoesNotExist,
    LoopDetected,
    TooManyHops,
    AddressIncomplete,
    Ambiguous,
    BusyHere,
    RequestTerminated,
    NotAcceptableHere,
    BadEvent,
    RequestPending,
    Undecipherable,
    SecurityAgreementRequired,
    ServerInternalError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    ServerTimeOut,
    VersionNotSupported,
    MessageTooLarge,
    PreconditionFailure,
    BusyEverywhere,
    Decline,
    DoesNotExistAnywhere,
    NotAcceptableGlobal,
    Unwanted,
    Other(u16, String),
}

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

impl StatusCode {
    pub fn code(&self) -> u16 {
        match self {
            Self::Trying => 100,
            Self::Ringing => 180,
            Self::CallIsBeingForwarded => 181,
            Self::Queued => 182,
            Self::SessionProgress => 183,
            Self::EarlyDialogTerminated => 199,
            Self::Ok => 200,
            Self::Accepted => 201,
            Self::NoNotification => 204,
            Self::MultipleChoices => 300,
            Self::MovedPermanently => 301,
            Self::MovedTemporarily => 302,
            Self::UseProxy => 305,
            Self::AlternativeService => 380,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::PaymentRequired => 402,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            Self::NotAcceptable => 406,
            Self::ProxyAuthenticationRequired => 407,
            Self::RequestTimeout => 408,
            Self::Conflict => 409,
            Self::Gone => 410,
            Self::LengthRequired => 411,
            Self::ConditionalRequestFailed => 412,
            Self::RequestEntityTooLarge => 413,
            Self::RequestUriTooLong => 414,
            Self::UnsupportedMediaType => 415,
            Self::UnsupportedUriScheme => 416,
            Self::UnknownResourcePriority => 417,
            Self::BadExtension => 420,
            Self::ExtensionRequired => 421,
            Self::SessionIntervalTooSmall => 422,
            Self::IntervalTooBrief => 423,
            Self::BadLocationInformation => 424,
            Self::UseIdentityHeader => 428,
            Self::ProvideReferrerIdentity => 429,
            Self::AnonymityDisallowed => 433,
            Self::BadIdentityInfo => 436,
            Self::UnsupportedCertificate => 437,
            Self::InvalidIdentityHeader => 438,
            Self::FirstHopLacksOutboundSupport => 439,
            Self::MaxBreadthExceeded => 440,
            Self::BadInfoPackage => 469,
            Self::ConsentNeeded => 470,
            Self::TemporarilyUnavailable => 480,
            Self::CallTransactionDoesNotExist => 481,
            Self::LoopDetected => 482,
            Self::TooManyHops => 483,
            Self::AddressIncomplete => 484,
            Self::Ambiguous => 485,
            Self::BusyHere => 486,
            Self::RequestTerminated => 487,
            Self::NotAcceptableHere => 488,
            Self::BadEvent => 489,
            Self::RequestPending => 491,
            Self::Undecipherable => 493,
            Self::SecurityAgreementRequired => 494,
            Self::ServerInternalError => 500,
            Self::NotImplemented => 501,
            Self::BadGateway => 502,
            Self::ServiceUnavailable => 503,
            Self::ServerTimeOut => 504,
            Self::VersionNotSupported => 505,
            Self::MessageTooLarge => 513,
            Self::PreconditionFailure => 580,
            Self::BusyEverywhere => 600,
            Self::Decline => 603,
            Self::DoesNotExistAnywhere => 604,
            Self::NotAcceptableGlobal => 606,
            Self::Unwanted => 607,
            Self::Other(code, _) => *code,
        }
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        Self::Ok
    }
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trying => write!(f, "100 Trying"),
            Self::Ringing => write!(f, "180 Ringing"),
            Self::CallIsBeingForwarded => write!(f, "180 CallIsBeingForwarded"),
            Self::Queued => write!(f, "182 Queued"),
            Self::SessionProgress => write!(f, "183 SessionProgress"),
            Self::EarlyDialogTerminated => write!(f, "199 EarlyDialogTerminated"),
            Self::Ok => write!(f, "200 OK"),
            Self::Accepted => write!(f, "201 Accepted"),
            Self::NoNotification => write!(f, "204 NoNotification"),
            Self::MultipleChoices => write!(f, "300 MultipleChoices"),
            Self::MovedPermanently => write!(f, "301 MovedPermanently"),
            Self::MovedTemporarily => write!(f, "302 MovedTemporarily"),
            Self::UseProxy => write!(f, "305 UseProxy"),
            Self::AlternativeService => write!(f, "380 AlternativeService"),
            Self::BadRequest => write!(f, "400 BadRequest"),
            Self::Unauthorized => write!(f, "401 Unauthorized"),
            Self::PaymentRequired => write!(f, "402 PaymentRequired"),
            Self::Forbidden => write!(f, "403 PaymentRequired"),
            Self::NotFound => write!(f, "404 NotFound"),
            Self::MethodNotAllowed => write!(f, "405 MethodNotAllowed"),
            Self::NotAcceptable => write!(f, "406 NotAcceptable"),
            Self::ProxyAuthenticationRequired => write!(f, "407 ProxyAuthenticationRequired"),
            Self::RequestTimeout => write!(f, "408 RequestTimeout"),
            Self::Conflict => write!(f, "409 Conflict"),
            Self::Gone => write!(f, "410 Gone"),
            Self::LengthRequired => write!(f, "411 LengthRequired"),
            Self::ConditionalRequestFailed => write!(f, "412 ConditionalRequestFailed"),
            Self::RequestEntityTooLarge => write!(f, "413 RequestEntityTooLarge"),
            Self::RequestUriTooLong => write!(f, "414 RequestUriTooLong"),
            Self::UnsupportedMediaType => write!(f, "415 UnsupportedMediaType"),
            Self::UnsupportedUriScheme => write!(f, "416 UnsupportedUriScheme"),
            Self::UnknownResourcePriority => write!(f, "417 UnknownResourcePriority"),
            Self::BadExtension => write!(f, "420 BadExtension"),
            Self::ExtensionRequired => write!(f, "421 ExtensionRequired"),
            Self::SessionIntervalTooSmall => write!(f, "422 SessionIntervalTooSmall"),
            Self::IntervalTooBrief => write!(f, "423 IntervalTooBrief"),
            Self::BadLocationInformation => write!(f, "424 BadLocationInformation"),
            Self::UseIdentityHeader => write!(f, "428 UseIdentityHeader"),
            Self::ProvideReferrerIdentity => write!(f, "429 ProvideReferrerIdentity"),
            Self::AnonymityDisallowed => write!(f, "433 AnonymityDisallowed"),
            Self::BadIdentityInfo => write!(f, "436 BadIdentityInfo"),
            Self::UnsupportedCertificate => write!(f, "437 UnsupportedCertificate"),
            Self::InvalidIdentityHeader => write!(f, "438 InvalidIdentityHeader"),
            Self::FirstHopLacksOutboundSupport => write!(f, "439 FirstHopLacksOutboundSupport"),
            Self::MaxBreadthExceeded => write!(f, "440 MaxBreadthExceeded"),
            Self::BadInfoPackage => write!(f, "469 BadInfoPackage"),
            Self::ConsentNeeded => write!(f, "470 ConsentNeeded"),
            Self::TemporarilyUnavailable => write!(f, "480 TemporarilyUnavailable"),
            Self::CallTransactionDoesNotExist => write!(f, "481 CallTransactionDoesNotExist"),
            Self::LoopDetected => write!(f, "482 LoopDetected"),
            Self::TooManyHops => write!(f, "483 TooManyHops"),
            Self::AddressIncomplete => write!(f, "484 AddressIncomplete"),
            Self::Ambiguous => write!(f, "485 Ambiguous"),
            Self::BusyHere => write!(f, "486 BusyHere"),
            Self::RequestTerminated => write!(f, "487 RequestTerminated"),
            Self::NotAcceptableHere => write!(f, "488 NotAcceptableHere"),
            Self::BadEvent => write!(f, "489 BadEvent"),
            Self::RequestPending => write!(f, "491 RequestPending"),
            Self::Undecipherable => write!(f, "493 Undecipherable"),
            Self::SecurityAgreementRequired => write!(f, "494 SecurityAgreementRequired"),
            Self::ServerInternalError => write!(f, "500 ServerInternalError"),
            Self::NotImplemented => write!(f, "501 NotImplemented"),
            Self::BadGateway => write!(f, "502 BadGateway"),
            Self::ServiceUnavailable => write!(f, "503 ServiceUnavailable"),
            Self::ServerTimeOut => write!(f, "504 ServerTimeOut"),
            Self::VersionNotSupported => write!(f, "505 VersionNotSupported"),
            Self::MessageTooLarge => write!(f, "513 MessageTooLarge"),
            Self::PreconditionFailure => write!(f, "580 PreconditionFailure"),
            Self::BusyEverywhere => write!(f, "600 BusyEverywhere"),
            Self::Decline => write!(f, "603 Decline"),
            Self::DoesNotExistAnywhere => write!(f, "604 DoesNotExistAnywhere"),
            Self::NotAcceptableGlobal => write!(f, "606 NotAcceptableGlobal"),
            Self::Unwanted => write!(f, "607 Unwanted"),
            Self::Other(code, reason) => write!(f, "{} {}", code, reason),
        }
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
            use std::str::from_utf8;

            match (from_utf8(self.code)?.parse::<u16>()?, self.reason) {
                (100, _) => Ok(StatusCode::Trying),
                (180, _) => Ok(StatusCode::Ringing),
                (181, _) => Ok(StatusCode::CallIsBeingForwarded),
                (182, _) => Ok(StatusCode::Queued),
                (183, _) => Ok(StatusCode::SessionProgress),
                (199, _) => Ok(StatusCode::EarlyDialogTerminated),
                (200, _) => Ok(StatusCode::Ok),
                (201, _) => Ok(StatusCode::Accepted),
                (204, _) => Ok(StatusCode::NoNotification),
                (300, _) => Ok(StatusCode::MultipleChoices),
                (301, _) => Ok(StatusCode::MovedPermanently),
                (302, _) => Ok(StatusCode::MovedTemporarily),
                (305, _) => Ok(StatusCode::UseProxy),
                (380, _) => Ok(StatusCode::AlternativeService),
                (400, _) => Ok(StatusCode::BadRequest),
                (401, _) => Ok(StatusCode::Unauthorized),
                (402, _) => Ok(StatusCode::PaymentRequired),
                (403, _) => Ok(StatusCode::Forbidden),
                (404, _) => Ok(StatusCode::NotFound),
                (405, _) => Ok(StatusCode::MethodNotAllowed),
                (406, _) => Ok(StatusCode::NotAcceptable),
                (407, _) => Ok(StatusCode::ProxyAuthenticationRequired),
                (408, _) => Ok(StatusCode::RequestTimeout),
                (409, _) => Ok(StatusCode::Conflict),
                (410, _) => Ok(StatusCode::Gone),
                (411, _) => Ok(StatusCode::LengthRequired),
                (412, _) => Ok(StatusCode::ConditionalRequestFailed),
                (413, _) => Ok(StatusCode::RequestEntityTooLarge),
                (414, _) => Ok(StatusCode::RequestUriTooLong),
                (415, _) => Ok(StatusCode::UnsupportedMediaType),
                (416, _) => Ok(StatusCode::UnsupportedUriScheme),
                (417, _) => Ok(StatusCode::UnknownResourcePriority),
                (420, _) => Ok(StatusCode::BadExtension),
                (421, _) => Ok(StatusCode::ExtensionRequired),
                (422, _) => Ok(StatusCode::SessionIntervalTooSmall),
                (423, _) => Ok(StatusCode::IntervalTooBrief),
                (424, _) => Ok(StatusCode::BadLocationInformation),
                (428, _) => Ok(StatusCode::UseIdentityHeader),
                (429, _) => Ok(StatusCode::ProvideReferrerIdentity),
                (433, _) => Ok(StatusCode::AnonymityDisallowed),
                (436, _) => Ok(StatusCode::BadIdentityInfo),
                (437, _) => Ok(StatusCode::UnsupportedCertificate),
                (438, _) => Ok(StatusCode::InvalidIdentityHeader),
                (439, _) => Ok(StatusCode::FirstHopLacksOutboundSupport),
                (440, _) => Ok(StatusCode::MaxBreadthExceeded),
                (469, _) => Ok(StatusCode::BadInfoPackage),
                (470, _) => Ok(StatusCode::ConsentNeeded),
                (480, _) => Ok(StatusCode::TemporarilyUnavailable),
                (481, _) => Ok(StatusCode::CallTransactionDoesNotExist),
                (482, _) => Ok(StatusCode::LoopDetected),
                (483, _) => Ok(StatusCode::TooManyHops),
                (484, _) => Ok(StatusCode::AddressIncomplete),
                (485, _) => Ok(StatusCode::Ambiguous),
                (486, _) => Ok(StatusCode::BusyHere),
                (487, _) => Ok(StatusCode::RequestTerminated),
                (488, _) => Ok(StatusCode::NotAcceptableHere),
                (489, _) => Ok(StatusCode::BadEvent),
                (491, _) => Ok(StatusCode::RequestPending),
                (493, _) => Ok(StatusCode::Undecipherable),
                (494, _) => Ok(StatusCode::SecurityAgreementRequired),
                (500, _) => Ok(StatusCode::ServerInternalError),
                (501, _) => Ok(StatusCode::NotImplemented),
                (502, _) => Ok(StatusCode::BadGateway),
                (503, _) => Ok(StatusCode::ServiceUnavailable),
                (504, _) => Ok(StatusCode::ServerTimeOut),
                (505, _) => Ok(StatusCode::VersionNotSupported),
                (513, _) => Ok(StatusCode::MessageTooLarge),
                (580, _) => Ok(StatusCode::PreconditionFailure),
                (600, _) => Ok(StatusCode::BusyEverywhere),
                (603, _) => Ok(StatusCode::Decline),
                (604, _) => Ok(StatusCode::DoesNotExistAnywhere),
                (606, _) => Ok(StatusCode::NotAcceptableGlobal),
                (607, _) => Ok(StatusCode::Unwanted),
                (code, reason) => Ok(StatusCode::Other(code, from_utf8(reason)?.into())),
            }
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
