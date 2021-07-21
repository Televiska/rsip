//! Module holding the [UntypedHeader] trait and all the untyped headers.
//! An untyped header is basically nothing more than a simple NewType around `String` with many
//! many helpful goodies.
//!
//! Some untyped headers have a [typed](super::typed) equivalent header, like [From] equivalent is
//! [typed::From](super::typed::From), [Via] equivalent is [typed::Via](super::typed::Via) etc. Those
//! untyped headers implement the [ToTypedHeader] trait.
//!
//! If a header is not implemented in Rsip, then you can use the `Other` variant of the
//! [Header](super::Header) enum.

use crate::Header;

pub mod accept;
pub mod accept_encoding;
pub mod accept_language;
pub mod alert_info;
pub mod allow;
pub mod authentication_info;
pub mod authorization;
pub mod call_id;
pub mod call_info;
pub mod contact;
pub mod content_disposition;
pub mod content_encoding;
pub mod content_language;
pub mod content_length;
pub mod content_type;
pub mod cseq;
pub mod date;
pub mod error_info;
pub mod event;
pub mod expires;
pub mod from;
pub mod in_reply_to;
pub mod max_forwards;
pub mod mime_version;
pub mod min_expires;
pub mod organization;
pub mod priority;
pub mod proxy_authenticate;
pub mod proxy_authorization;
pub mod proxy_require;
pub mod record_route;
pub mod reply_to;
pub mod require;
pub mod retry_after;
pub mod route;
pub mod server;
pub mod subject;
pub mod subscription_state;
pub mod supported;
pub mod timestamp;
pub mod to;
pub mod unsupported;
pub mod user_agent;
pub mod via;
pub mod warning;
pub mod www_authenticate;

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

pub trait UntypedHeader<'a>:
    std::fmt::Debug
    + std::fmt::Display
    + std::cmp::PartialEq
    + std::cmp::Eq
    + std::clone::Clone
    + std::convert::From<String>
    + std::convert::Into<String>
    + std::convert::From<&'a str>
    + std::convert::Into<Header>
{
    fn new(value: impl Into<String>) -> Self;
    fn value(&self) -> &str;
    fn replace(&mut self, new_value: impl Into<String>);
}

pub trait ToTypedHeader<'a>:
    UntypedHeader<'a> + std::convert::TryInto<Self::Typed, Error = crate::Error>
{
    type Typed: crate::headers::typed::TypedHeader<'a> + Into<Self>;

    fn typed(&self) -> Result<Self::Typed, crate::Error> {
        self.clone().try_into()
    }
    fn into_typed(self) -> Result<Self::Typed, crate::Error> {
        self.try_into()
    }
}
