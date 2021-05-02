use crate::{Error, NomError};
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
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
    Other(u16),
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
        let code = Into::<u16>::into(*self);
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

    pub fn parse(tokenizer: Tokenizer) -> Result<Self, Error> {
        use std::str::from_utf8;

        Ok(from_utf8(tokenizer.code)?.parse::<u16>()?.into())
    }
}

impl<'a> TryFrom<Tokenizer<'a>> for StatusCode {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Error> {
        Self::parse(tokenizer)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub code: &'a [u8],
    pub reason: Option<&'a [u8]>,
}

impl<'a> From<&'a [u8]> for Tokenizer<'a> {
    fn from(code: &'a [u8]) -> Self {
        Self { code, reason: None }
    }
}

impl<'a> From<(&'a [u8], &'a [u8])> for Tokenizer<'a> {
    fn from(tuple: (&'a [u8], &'a [u8])) -> Self {
        Self {
            code: tuple.0,
            reason: Some(tuple.1),
        }
    }
}

impl<'a> From<(&'a [u8], Option<&'a [u8]>)> for Tokenizer<'a> {
    fn from(tuple: (&'a [u8], Option<&'a [u8]>)) -> Self {
        Self {
            code: tuple.0,
            reason: tuple.1,
        }
    }
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
        use crate::parser_utils::opt_sp;
        use nom::{character::complete::digit1, sequence::tuple};

        let (rem, (code, _)) = tuple((digit1, opt_sp))(part)?;

        Ok((rem, code.into()))
    }

    pub fn tokenize_with_reason(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
        use nom::{
            bytes::complete::{tag, take_until},
            character::complete::digit1,
            sequence::tuple,
        };

        let (rem, (code, _, reason, _)) =
            tuple((digit1, tag(" "), take_until("\r\n"), tag("\r\n")))(part)?;

        Ok((
            rem,
            Self {
                code,
                reason: Some(reason),
            },
        ))
    }
}

impl From<StatusCode> for u16 {
    fn from(from: StatusCode) -> u16 {
        use StatusCode::*;

        match from {
            Trying => 100,
            Ringing => 180,
            CallIsBeingForwarded => 181,
            Queued => 182,
            SessionProgress => 183,
            EarlyDialogTerminated => 199,
            Ok => 200,
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
            Unwanted => 607,
            Other(code) => code,
        }
    }
}

impl From<u16> for StatusCode {
    fn from(from: u16) -> Self {
        use StatusCode::*;

        match from {
            100 => Trying,
            180 => Ringing,
            181 => CallIsBeingForwarded,
            182 => Queued,
            183 => SessionProgress,
            199 => EarlyDialogTerminated,
            200 => Ok,
            201 => Accepted,
            204 => NoNotification,
            300 => MultipleChoices,
            301 => MovedPermanently,
            302 => MovedTemporarily,
            305 => UseProxy,
            380 => AlternativeService,
            400 => BadRequest,
            401 => Unauthorized,
            402 => PaymentRequired,
            403 => Forbidden,
            404 => NotFound,
            405 => MethodNotAllowed,
            406 => NotAcceptable,
            407 => ProxyAuthenticationRequired,
            408 => RequestTimeout,
            409 => Conflict,
            410 => Gone,
            411 => LengthRequired,
            412 => ConditionalRequestFailed,
            413 => RequestEntityTooLarge,
            414 => RequestUriTooLong,
            415 => UnsupportedMediaType,
            416 => UnsupportedUriScheme,
            417 => UnknownResourcePriority,
            420 => BadExtension,
            421 => ExtensionRequired,
            422 => SessionIntervalTooSmall,
            423 => IntervalTooBrief,
            424 => BadLocationInformation,
            428 => UseIdentityHeader,
            429 => ProvideReferrerIdentity,
            433 => AnonymityDisallowed,
            436 => BadIdentityInfo,
            437 => UnsupportedCertificate,
            438 => InvalidIdentityHeader,
            439 => FirstHopLacksOutboundSupport,
            440 => MaxBreadthExceeded,
            469 => BadInfoPackage,
            470 => ConsentNeeded,
            480 => TemporarilyUnavailable,
            481 => CallTransactionDoesNotExist,
            482 => LoopDetected,
            483 => TooManyHops,
            484 => AddressIncomplete,
            485 => Ambiguous,
            486 => BusyHere,
            487 => RequestTerminated,
            488 => NotAcceptableHere,
            489 => BadEvent,
            491 => RequestPending,
            493 => Undecipherable,
            494 => SecurityAgreementRequired,
            500 => ServerInternalError,
            501 => NotImplemented,
            502 => BadGateway,
            503 => ServiceUnavailable,
            504 => ServerTimeOut,
            505 => VersionNotSupported,
            513 => MessageTooLarge,
            580 => PreconditionFailure,
            600 => BusyEverywhere,
            603 => Decline,
            604 => DoesNotExistAnywhere,
            606 => NotAcceptableGlobal,
            607 => Unwanted,
            code => Other(code),
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
        write!(f, "{}", Into::<u16>::into(*self))
    }
}
