#[doc(hidden)]
pub use tokenizer::Tokenizer;

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

        fn match_from<'a>(value: &'a [u8]) -> Result<Method, crate::Error> {
            use nom::{
                branch::alt,
                bytes::complete::{tag_no_case},
                combinator::{rest, map},
                error::VerboseError
            };

            let (_, method) = alt::<_, _, VerboseError<&'a [u8]>, _>((
                $(
                    map(tag_no_case(stringify!($name)), |_| Ok(Method::$name)),
                )*
                map(rest, |_| Err(crate::Error::ParseError(format!("Invalid method `{:?}`", value))))
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

#[doc(hidden)]
pub mod tokenizer {
    use super::Method;
    use crate::{Error, NomError};
    use std::convert::TryInto;

    impl<'a> TryInto<Method> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<Method, Error> {
            super::match_from(self.value)
        }
    }

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
        pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
            use crate::parser_utils::{create_error_for, is_token, opt_sp};
            use nom::{bytes::complete::take_while, sequence::tuple};

            let (rem, (method, _)) = tuple((take_while(is_token), opt_sp))(part)?;
            //TODO: helpful to return early in case we parse a response but maybe it should not
            //be checked here though
            if method.starts_with(b"SIP/") {
                return Err(create_error_for(method, "SIP version found instead"));
            }

            Ok((rem, method.into()))
        }
    }
}
