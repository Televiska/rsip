//! Module holding the [TypedHeader] trait and all typed headers. A typed header is basically any
//! header that holds something more complex than an opque string (like `Call-ID`) or a number
//! (like `Expires`). New typed headers will be added along the way for newer RFCs like
//! [PASSporT](https://datatracker.ietf.org/doc/html/rfc8224),
//! [SHAKEN](https://datatracker.ietf.org/doc/html/rfc8588),
//! [push notifications](https://datatracker.ietf.org/doc/html/rfc8599) etc
//!

pub mod alert_info;
pub mod allow;
pub mod authorization;
pub mod contact;
pub mod cseq;
pub mod from;
pub mod to;
pub mod tokenizers;
pub mod via;
pub mod www_authenticate;

pub use alert_info::AlertInfo;
pub use allow::Allow;
pub use authorization::Authorization;
pub use contact::Contact;
pub use cseq::CSeq;
pub use from::From;
pub use to::To;
pub use via::Via;
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
