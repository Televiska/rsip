pub use tokenizer::Tokenizer;

pub mod maddr;
pub mod method;
pub mod transport;
pub mod ttl;
pub mod user;

pub use maddr::Maddr;
pub use method::Method;
pub use transport::Transport;
pub use ttl::Ttl;
pub use user::User;

use macros::{Display, FromIntoInner, FromStr, HasValue};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Param {
    Transport(Transport),
    User(User),
    Method(Method),
    Ttl(Ttl),
    Maddr(Maddr),
    Lr,
    //TODO: value should be an option here
    Other(OtherParam, Option<OtherParamValue>),
}

#[derive(HasValue, Display, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct OtherParam(String);
#[derive(HasValue, Display, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct OtherParamValue(String);

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
                (s, Some(v)) if s.eq_ignore_ascii_case("branch") => {
                    Ok(Param::Transport(Transport::new(v)))
                }
                (s, Some(v)) if s.eq_ignore_ascii_case("user") => Ok(Param::User(User::new(v))),
                (s, Some(v)) if s.eq_ignore_ascii_case("method") => {
                    Ok(Param::Method(Method::new(v)))
                }
                (s, Some(v)) if s.eq_ignore_ascii_case("ttl") => Ok(Param::Ttl(Ttl::new(v))),
                (s, Some(v)) if s.eq_ignore_ascii_case("maddr") => Ok(Param::Maddr(Maddr::new(v))),
                (s, None) if s.eq_ignore_ascii_case("lr") => Ok(Param::Lr),
                (s, v) => Ok(Param::Other(s.into(), v.map(Into::into))),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Utf8Tokenizer)]
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
