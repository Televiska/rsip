#[doc(hidden)]
pub use tokenizer::Tokenizer;

pub mod branch;
pub mod expires;
pub mod maddr;
pub mod method;
pub mod q;
pub mod received;
pub mod tag;
pub mod ttl;
pub mod user;

pub use branch::Branch;
pub use expires::Expires;
pub use maddr::Maddr;
pub use method::Method;
pub use q::Q;
pub use received::Received;
pub use tag::Tag;
pub use ttl::Ttl;
pub use user::User;

use crate::{Error, Transport};
use rsip_derives::NewType;
use std::convert::TryInto;

/// This enum holds all the possible parameters found in SIP(S) URIs, and headers like `From`,
/// `To`, `Contact`, `Via` etc. For better safety, we should probably define different param
/// enums for each of those cases since, for instance, a `branch` parameter should not appear
/// in a `Contact` header, however we have it in the same enum for simplicity for now and delegate
/// this safety to the user.
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

#[derive(NewType, Debug, PartialEq, Eq, Clone)]
pub struct OtherParam(String);
#[derive(NewType, Debug, PartialEq, Eq, Clone)]
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

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a str, char>> for Param {
    type Error = Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a str, char>) -> Result<Self, Self::Error> {
        (tokenizer.name, tokenizer.value).try_into()
    }
}

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a [u8], u8>> for Param {
    type Error = Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a [u8], u8>) -> Result<Self, Self::Error> {
        use std::str::from_utf8;

        Self::try_from(Tokenizer::from((
            from_utf8(tokenizer.name)?,
            tokenizer.value.map(from_utf8).transpose()?,
        )))
    }
}

impl<'a> std::convert::TryFrom<(&'a str, Option<&'a str>)> for Param {
    type Error = Error;

    fn try_from(from: (&'a str, Option<&'a str>)) -> Result<Self, Self::Error> {
        use std::str::FromStr;

        match (from.0, from.1) {
            (s, Some(v)) if s.eq_ignore_ascii_case("transport") => {
                Ok(Param::Transport(Transport::from_str(v)?))
            }
            (s, Some(v)) if s.eq_ignore_ascii_case("user") => Ok(Param::User(User::new(v))),
            (s, Some(v)) if s.eq_ignore_ascii_case("method") => Ok(Param::Method(Method::new(v))),
            (s, Some(v)) if s.eq_ignore_ascii_case("ttl") => Ok(Param::Ttl(Ttl::new(v))),
            (s, Some(v)) if s.eq_ignore_ascii_case("maddr") => Ok(Param::Maddr(Maddr::new(v))),
            (s, Some(v)) if s.eq_ignore_ascii_case("branch") => Ok(Param::Branch(Branch::new(v))),
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

#[doc(hidden)]
pub mod tokenizer {
    use crate::{AbstractInput, AbstractInputItem, GResult, GenericNomError, TokenizerError};
    use std::marker::PhantomData;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        pub name: T,
        pub value: Option<T>,
        phantom1: PhantomData<&'a T>,
        phantom2: PhantomData<I>,
    }

    impl<'a, T, I> From<(T, Option<T>)> for Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        fn from(from: (T, Option<T>)) -> Self {
            Self {
                name: from.0,
                value: from.1,
                phantom1: PhantomData,
                phantom2: PhantomData,
            }
        }
    }

    impl<'a, T, I> Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        pub fn tokenize(part: T) -> GResult<T, Self> {
            use nom::{
                bytes::complete::{tag, take_while},
                combinator::{map, opt},
                sequence::tuple,
            };

            let (rem, (_, name, value)) = tuple((
                tag(";"),
                take_while(I::is_alphabetic),
                opt(map(tuple((tag("="), take_while(I::is_token))), |t| t.1)),
            ))(part)
            .map_err(|_: GenericNomError<'a, T>| {
                TokenizerError::from(("uri param", part)).into()
            })?;

            Ok((rem, (name, value).into()))
        }
    }
}
