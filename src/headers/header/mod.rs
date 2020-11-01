mod accept;
mod accept_encoding;
mod accept_language;
mod alert_info;
mod allow;
mod authentication_info;
mod authorization;
mod call_id;
mod call_info;
mod contact;
mod content_disposition;
mod content_encoding;
mod content_language;
mod content_length;
mod content_type;
mod cseq;
mod date;
mod error_info;
mod event;
mod expires;
mod from;
mod in_reply_to;
mod max_forwards;
mod mime_version;
mod min_expires;
pub mod named;
mod organization;
mod priority;
mod proxy_authenticate;
mod proxy_authorization;
mod proxy_require;
mod record_route;
mod reply_to;
mod require;
mod retry_after;
mod route;
mod server;
mod subject;
mod subscription_state;
mod supported;
mod timestamp;
mod to;
mod unsupported;
mod user_agent;
mod via;
mod warning;
mod www_authenticate;
mod x_fs_sending_message;

pub use accept::Accept;
pub use accept_encoding::AcceptEncoding;
pub use accept_language::AcceptLanguage;
pub use alert_info::AlertInfo;
pub use allow::Allow;
pub use authentication_info::AuthenticationInfo;
pub use authorization::Authorization;
pub use call_id::CallId;
pub use call_info::CallInfo;
pub use contact::Contact;
pub use content_disposition::ContentDisposition;
pub use content_encoding::ContentEncoding;
pub use content_language::ContentLanguage;
pub use content_length::ContentLength;
pub use content_type::ContentType;
pub use cseq::CSeq;
pub use date::Date;
pub use error_info::ErrorInfo;
pub use event::Event;
pub use expires::Expires;
pub use from::From;
pub use in_reply_to::InReplyTo;
pub use max_forwards::MaxForwards;
pub use mime_version::MimeVersion;
pub use min_expires::MinExpires;
pub use named::{ContactParam, GenValue, NamedHeader, NamedParam, NamedParams};
pub use organization::Organization;
pub use priority::Priority;
pub use proxy_authenticate::ProxyAuthenticate;
pub use proxy_authorization::ProxyAuthorization;
pub use proxy_require::ProxyRequire;
pub use record_route::RecordRoute;
pub use reply_to::ReplyTo;
pub use require::Require;
pub use retry_after::RetryAfter;
pub use route::Route;
pub use server::Server;
pub use subject::Subject;
pub use subscription_state::SubscriptionState;
pub use supported::Supported;
pub use timestamp::Timestamp;
pub use to::To;
pub use unsupported::Unsupported;
pub use user_agent::UserAgent;
pub use via::Via;
pub use warning::Warning;
pub use www_authenticate::WwwAuthenticate;
pub use x_fs_sending_message::XFsSendingMessage;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Header {
    To(To),
    Contact(Contact),
    From(From),
    ReplyTo(ReplyTo),
    CSeq(CSeq),
    MaxForwards(MaxForwards),
    Event(Event),
    Expires(Expires),
    Accept(Accept),
    ContentLength(ContentLength),
    Allow(Allow),
    UserAgent(UserAgent),
    CallId(CallId),
    ContentType(ContentType),
    ContentLanguage(ContentLanguage),
    ContentEncoding(ContentEncoding),
    AcceptLanguage(AcceptLanguage),
    AcceptEncoding(AcceptEncoding),
    AlertInfo(AlertInfo),
    ErrorInfo(ErrorInfo),
    AuthenticationInfo(AuthenticationInfo),
    Authorization(Authorization),
    CallInfo(CallInfo),
    InReplyTo(InReplyTo),
    ContentDisposition(ContentDisposition),
    Date(Date),
    MinExpires(MinExpires),
    MimeVersion(MimeVersion),
    Organization(Organization),
    ProxyAuthenticate(ProxyAuthenticate),
    ProxyAuthorization(ProxyAuthorization),
    ProxyRequire(ProxyRequire),
    Require(Require),
    RetryAfter(RetryAfter),
    Route(Route),
    Subject(Subject),
    SubscriptionState(SubscriptionState),
    RecordRoute(RecordRoute),
    Server(Server),
    Supported(Supported),
    Timestamp(Timestamp),
    Unsupported(Unsupported),
    Warning(Warning),
    Via(Via),
    Priority(Priority),
    WwwAuthenticate(WwwAuthenticate),
    XFsSendingMessage(XFsSendingMessage),
    Other(String, String),
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<libsip::Header>::into(self.clone()))
    }
}

impl Into<libsip::Header> for Header {
    fn into(self) -> libsip::Header {
        match self {
            Self::To(inner) => inner.into(),
            Self::Contact(inner) => inner.into(),
            Self::From(inner) => inner.into(),
            Self::ReplyTo(inner) => inner.into(),
            Self::CSeq(inner) => inner.into(),
            Self::MaxForwards(inner) => inner.into(),
            Self::Event(inner) => inner.into(),
            Self::Expires(inner) => inner.into(),
            Self::Accept(inner) => inner.into(),
            Self::ContentLength(inner) => inner.into(),
            Self::Allow(inner) => inner.into(),
            Self::UserAgent(inner) => inner.into(),
            Self::CallId(inner) => inner.into(),
            Self::ContentType(inner) => inner.into(),
            Self::ContentLanguage(inner) => inner.into(),
            Self::ContentEncoding(inner) => inner.into(),
            Self::AcceptLanguage(inner) => inner.into(),
            Self::AcceptEncoding(inner) => inner.into(),
            Self::AlertInfo(inner) => inner.into(),
            Self::ErrorInfo(inner) => inner.into(),
            Self::AuthenticationInfo(inner) => inner.into(),
            Self::Authorization(inner) => inner.into(),
            Self::CallInfo(inner) => inner.into(),
            Self::InReplyTo(inner) => inner.into(),
            Self::ContentDisposition(inner) => inner.into(),
            Self::Date(inner) => inner.into(),
            Self::MinExpires(inner) => inner.into(),
            Self::MimeVersion(inner) => inner.into(),
            Self::Organization(inner) => inner.into(),
            Self::ProxyAuthenticate(inner) => inner.into(),
            Self::ProxyAuthorization(inner) => inner.into(),
            Self::ProxyRequire(inner) => inner.into(),
            Self::Require(inner) => inner.into(),
            Self::RetryAfter(inner) => inner.into(),
            Self::Route(inner) => inner.into(),
            Self::Subject(inner) => inner.into(),
            Self::SubscriptionState(inner) => inner.into(),
            Self::RecordRoute(inner) => inner.into(),
            Self::Server(inner) => inner.into(),
            Self::Supported(inner) => inner.into(),
            Self::Timestamp(inner) => inner.into(),
            Self::Unsupported(inner) => inner.into(),
            Self::Warning(inner) => inner.into(),
            Self::Via(inner) => inner.into(),
            Self::Priority(inner) => inner.into(),
            Self::WwwAuthenticate(inner) => inner.into(),
            Self::XFsSendingMessage(inner) => inner.into(),
            Self::Other(key, value) => libsip::Header::Other(key, value),
        }
    }
}

impl std::convert::From<libsip::Header> for Header {
    fn from(from: libsip::Header) -> Self {
        use std::convert::TryInto;

        match from {
            libsip::Header::To(inner) => Self::To(inner.into()),
            libsip::Header::Contact(inner) => Self::Contact(inner.into()),
            libsip::Header::From(inner) => Self::From(inner.into()),
            libsip::Header::ReplyTo(inner) => Self::ReplyTo(inner.into()),
            libsip::Header::CSeq(seq, method) => Self::CSeq((seq, method.into()).into()),
            libsip::Header::MaxForwards(inner) => Self::MaxForwards(inner.into()),
            libsip::Header::Event(inner) => Self::Event(inner.into()),
            libsip::Header::Expires(inner) => Self::Expires(inner.into()),
            libsip::Header::Accept(inner) => Self::Other("accept".into(), format!("{:?}", inner)),
            libsip::Header::ContentLength(inner) => Self::ContentLength(inner.into()),
            libsip::Header::Allow(inner) => Self::Allow(inner.into()),
            libsip::Header::UserAgent(inner) => Self::UserAgent(inner.into()),
            libsip::Header::CallId(inner) => Self::CallId(inner.into()),
            libsip::Header::ContentType(inner) => Self::ContentType(inner.into()),
            libsip::Header::ContentLanguage(inner) => Self::ContentLanguage(inner.into()),
            libsip::Header::ContentEncoding(inner) => Self::ContentEncoding(inner.into()),
            libsip::Header::AcceptLanguage(inner) => Self::AcceptLanguage(inner.into()),
            libsip::Header::AcceptEncoding(inner) => Self::AcceptEncoding(inner.into()),
            libsip::Header::AlertInfo(inner) => Self::AlertInfo(inner.into()),
            libsip::Header::ErrorInfo(inner) => Self::ErrorInfo(inner.into()),
            libsip::Header::AuthenticationInfo(inner) => Self::AuthenticationInfo(inner.into()),
            libsip::Header::Authorization(inner) => Self::Authorization(
                inner
                    .try_into()
                    .expect("convert libsip authorization to rsip authorization"),
            ),
            libsip::Header::CallInfo(inner) => Self::CallInfo(inner.into()),
            libsip::Header::InReplyTo(inner) => Self::InReplyTo(inner.into()),
            libsip::Header::ContentDisposition(inner) => Self::ContentDisposition(inner.into()),
            libsip::Header::Date(inner) => Self::Date(inner.into()),
            libsip::Header::MinExpires(inner) => Self::MinExpires(inner.into()),
            libsip::Header::MimeVersion(inner) => Self::MimeVersion(inner.into()),
            libsip::Header::Organization(inner) => Self::Organization(inner.into()),
            libsip::Header::ProxyAuthenticate(inner) => Self::ProxyAuthenticate(inner.into()),
            libsip::Header::ProxyAuthorization(inner) => Self::ProxyAuthorization(inner.into()),
            libsip::Header::ProxyRequire(inner) => Self::ProxyRequire(inner.into()),
            libsip::Header::Require(inner) => Self::Require(inner.into()),
            libsip::Header::RetryAfter(inner) => Self::RetryAfter(inner.into()),
            libsip::Header::Route(inner) => Self::Route(inner.into()),
            libsip::Header::Subject(inner) => Self::Subject(inner.into()),
            libsip::Header::SubscriptionState(inner) => Self::SubscriptionState(inner.into()),
            libsip::Header::RecordRoute(inner) => Self::RecordRoute(inner.into()),
            libsip::Header::Server(inner) => Self::Server(inner.into()),
            libsip::Header::Supported(inner) => Self::Supported(inner.into()),
            libsip::Header::Timestamp(inner) => Self::Timestamp(inner.into()),
            libsip::Header::Unsupported(inner) => Self::Unsupported(inner.into()),
            libsip::Header::Warning(inner) => Self::Warning(inner.into()),
            libsip::Header::Via(inner) => Self::Via(inner.into()),
            libsip::Header::Priority(inner) => Self::Priority(inner.into()),
            libsip::Header::WwwAuthenticate(_) => {
                panic!("convert libsip WwwAuthenticate to rsip is not supported")
            }
            libsip::Header::XFsSendingMessage(inner) => Self::XFsSendingMessage(inner.into()),
            libsip::Header::Other(key, value) => Self::Other(key, value),
        }
    }
}
