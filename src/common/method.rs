#[doc(hidden)]
pub use tokenizer::Tokenizer;

use crate::Error;
use std::convert::TryFrom;

macro_rules! create_methods {
    ($($name:ident),*) => {

        /// The SIP [Request](super::super::Request) method.
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum Method {
            $(
                $name,
            )*
        }

        impl Method {
            pub fn all() -> Vec<Method> {
                vec![
                    $(
                        Self::$name,
                    )*
                ]
            }
        }

        impl std::fmt::Display for Method {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$name => write!(f, "{}", stringify!($name).to_uppercase()),
                    )*
                }
            }
        }

        fn match_from<'a>(value: &'a [u8]) -> Result<Method, Error> {
            use nom::{
                branch::alt,
                bytes::complete::{tag_no_case},
                combinator::{rest, map},
            };
            use bstr::ByteSlice;

            let (_, method) = alt((
                $(
                    map(tag_no_case(stringify!($name)), |_| Ok(Method::$name)),
                )*
                map(rest, |_| Err(Error::ParseError(format!("invalid method `{}`", value.as_bstr()))))
            ))(value)?;

            method
        }
    }
}

create_methods!(
    Ack, Bye, Cancel, Info, Invite, Message, Notify, Options, PRack, Publish, Refer, Register,
    Subscribe, Update
);

//TODO: not ideal performance here
impl std::str::FromStr for Method {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use std::convert::TryInto;

        tokenizer::Tokenizer {
            value: s.as_bytes(),
        }
        .try_into()
    }
}

impl<'a> TryFrom<Tokenizer<'a>> for Method {
    type Error = Error;

    fn try_from(from: Tokenizer<'a>) -> Result<Self, Self::Error> {
        match_from(from.value)
    }
}

#[doc(hidden)]
pub mod tokenizer {
    use crate::{IResult, TokenizerError};

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Tokenizer<'a> {
        pub value: &'a [u8],
    }

    impl<'a> From<&'a [u8]> for Tokenizer<'a> {
        fn from(value: &'a [u8]) -> Self {
            Self { value }
        }
    }

    impl<'a> Tokenizer<'a> {
        //works for request line
        pub fn tokenize(part: &'a [u8]) -> IResult<Self> {
            use crate::parser_utils::{is_token, opt_sp};
            use nom::{bytes::complete::take_while1, sequence::tuple};

            let (rem, (method, _)) = tuple((take_while1(is_token), opt_sp))(part)
                .map_err(|_| TokenizerError::from(("method", part)).into())?;
            //TODO: helpful to return early in case we parse a response but maybe it should not
            //be checked here though
            if method.starts_with(b"SIP/") {
                return TokenizerError::from(("method", part)).into();
            }

            Ok((rem, method.into()))
        }
    }
}
