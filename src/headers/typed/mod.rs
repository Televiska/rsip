pub mod authorization;
pub mod contact;
pub mod cseq;
pub mod from;
pub mod to;
pub mod via;
pub mod www_authenticate;

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
