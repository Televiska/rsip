//! Module holding the [TypedHeader] trait and all typed headers. A typed header is basically any
//! header that holds something more complex than an opque string (like `Call-ID`) or a number
//! (like `Expires`). New typed headers will be added along the way for newer RFCs like
//! [PASSporT](https://datatracker.ietf.org/doc/html/rfc8224),
//! [SHAKEN](https://datatracker.ietf.org/doc/html/rfc8588),
//! [push notifications](https://datatracker.ietf.org/doc/html/rfc8599) etc
//!

//This is not a header but a common helper
pub mod media_type;
pub use media_type::MediaType;

pub mod accept;
pub mod alert_info;
pub mod allow;
pub mod authentication_info;
pub mod authorization;
pub mod call_info;
pub mod contact;
pub mod content_disposition;
pub mod content_type;
pub mod cseq;
pub mod error_info;
pub mod from;
pub mod in_reply_to;
pub mod priority;
pub mod proxy_authenticate;
pub mod proxy_authorization;
pub mod record_route;
pub mod reply_to;
pub mod route;
pub mod to;
pub mod tokenizers;
pub mod via;
pub mod warning;
pub mod www_authenticate;

pub use accept::Accept;
pub use alert_info::AlertInfo;
pub use allow::Allow;
pub use authentication_info::AuthenticationInfo;
pub use authorization::Authorization;
pub use call_info::CallInfo;
pub use contact::Contact;
pub use content_disposition::ContentDisposition;
pub use content_type::ContentType;
pub use cseq::CSeq;
pub use error_info::ErrorInfo;
pub use from::From;
pub use in_reply_to::InReplyTo;
pub use priority::Priority;
pub use proxy_authenticate::ProxyAuthenticate;
pub use proxy_authorization::ProxyAuthorization;
pub use record_route::RecordRoute;
pub use reply_to::ReplyTo;
pub use route::Route;
pub use to::To;
pub use via::Via;
pub use warning::Warning;
pub use www_authenticate::WwwAuthenticate;

pub trait TypedHeader<'a>:
    std::fmt::Debug
    + std::fmt::Display
    + std::cmp::PartialEq
    + std::cmp::Eq
    + std::clone::Clone
    + std::convert::TryFrom<Self::Tokenizer, Error = crate::Error>
    + Into<String>
    + Into<crate::Header>
{
    type Tokenizer: Tokenize<'a>;
}

pub trait Tokenize<'a> {
    fn tokenize(part: &'a str) -> Result<Self, crate::Error>
    where
        Self: Sized;
}
