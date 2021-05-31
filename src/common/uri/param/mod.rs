pub use tokenizer::Tokenizer;

pub mod branch;
pub mod expires;
pub mod maddr;
pub mod method;
pub mod q;
pub mod received;
pub mod tag;
pub mod transport;
pub mod ttl;
pub mod user;

pub use branch::Branch;
pub use expires::Expires;
pub use maddr::Maddr;
pub use method::Method;
pub use q::Q;
pub use received::Received;
pub use tag::Tag;
pub use transport::Transport;
pub use ttl::Ttl;
pub use user::User;

use macros::{FromIntoInner, FromStr, HasValue, ValueDisplay};

//TODO: move out Via/From/etc params from here, but keep the same tokenizer
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Param {
    Transport(Transport),
    User(User),
    //TODO: should use regular method here
    Method(Method),
    Ttl(Ttl),
    Maddr(Maddr),
    Lr,
    Branch(Branch),     //param belonging to Via header but added here for simplicity
    Received(Received), //param belonging to Via header but added here for simplicity
    Tag(Tag),           //param belonging to From header but added here for simplicity
    Expires(Expires),   //param belonging to Contact header but added here for simplicity
    Q(Q),               //param belonging to Contact header but added here for simplicity
    Other(OtherParam, Option<OtherParamValue>),
}

#[derive(HasValue, ValueDisplay, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct OtherParam(String);
#[derive(HasValue, ValueDisplay, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct OtherParamValue(String);

impl std::fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Transport(transport) => write!(f, ";transport={}", transport),
            Self::User(user) => write!(f, ";user={}", user),
            Self::Method(method) => write!(f, ";method={}", method),
            Self::Ttl(ttl) => write!(f, ";ttl={}", ttl),
            Self::Maddr(maddr) => write!(f, ";maddr={}", maddr),
            Self::Lr => write!(f, ";lr"),
            Self::Branch(branch) => write!(f, ";branch={}", branch),
            Self::Received(received) => write!(f, ";received={}", received),
            Self::Tag(tag) => write!(f, ";tag={}", tag),
            Self::Expires(expires) => write!(f, ";expires={}", expires),
            Self::Q(q) => write!(f, ";q={}", q),
            Self::Other(name, Some(value)) => write!(f, ";{}={}", name, value),
            Self::Other(name, None) => write!(f, ";{}", name),
        }
    }
}

pub mod tokenizer {
    use super::*;
    use crate::{Error, NomError};
    use macros::Utf8Tokenizer;
    use std::convert::TryInto;

    impl<'a> TryInto<Param> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<Param, Error> {
            let tokenizer: Utf8Tokenizer = self.try_into()?;

            match (tokenizer.name, tokenizer.value) {
                (s, Some(v)) if s.eq_ignore_ascii_case("transport") => {
                    Ok(Param::Transport(Transport::new(v)))
                }
                (s, Some(v)) if s.eq_ignore_ascii_case("user") => Ok(Param::User(User::new(v))),
                (s, Some(v)) if s.eq_ignore_ascii_case("method") => {
                    Ok(Param::Method(Method::new(v)))
                }
                (s, Some(v)) if s.eq_ignore_ascii_case("ttl") => Ok(Param::Ttl(Ttl::new(v))),
                (s, Some(v)) if s.eq_ignore_ascii_case("maddr") => Ok(Param::Maddr(Maddr::new(v))),
                (s, Some(v)) if s.eq_ignore_ascii_case("branch") => {
                    Ok(Param::Branch(Branch::new(v)))
                }
                (s, Some(v)) if s.eq_ignore_ascii_case("received") => {
                    Ok(Param::Received(Received::new(v)))
                }
                (s, Some(v)) if s.eq_ignore_ascii_case("tag") => Ok(Param::Tag(Tag::new(v))),
                (s, Some(v)) if s.eq_ignore_ascii_case("expires") => {
                    Ok(Param::Expires(Expires::new(v)))
                }
                (s, Some(v)) if s.eq_ignore_ascii_case("q") => Ok(Param::Q(Q::new(v))),
                (s, None) if s.eq_ignore_ascii_case("lr") => Ok(Param::Lr),
                (s, v) => Ok(Param::Other(s.into(), v.map(Into::into))),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Utf8Tokenizer, Clone)]
    pub struct Tokenizer<'a> {
        pub name: &'a [u8],
        pub value: Option<&'a [u8]>,
    }

    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
            use crate::parser_utils::is_token;
            use nom::{
                bytes::complete::{tag, take_while},
                character::is_alphabetic,
                combinator::{map, opt},
                sequence::tuple,
            };

            let (rem, (_, name, value)) = tuple((
                tag(";"),
                take_while(is_alphabetic),
                opt(map(tuple((tag("="), take_while(is_token))), |t| t.1)),
            ))(part)?;

            Ok((rem, (name, value).into()))
        }
    }

    impl<'a> std::convert::From<(&'a [u8], Option<&'a [u8]>)> for Tokenizer<'a> {
        fn from(tuple: (&'a [u8], Option<&'a [u8]>)) -> Self {
            Self {
                name: tuple.0,
                value: tuple.1,
            }
        }
    }
}
