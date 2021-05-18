pub use tokenizer::Tokenizer;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Method {
    Ack,
    Bye,
    Cancel,
    Info,
    Invite,
    Message,
    Notify,
    Options,
    PRack,
    Publish,
    Refer,
    Register,
    Subscribe,
    Update,
}

impl Method {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Ack,
            Self::Bye,
            Self::Cancel,
            Self::Info,
            Self::Invite,
            Self::Message,
            Self::Notify,
            Self::Options,
            Self::PRack,
            Self::Publish,
            Self::Refer,
            Self::Register,
            Self::Subscribe,
            Self::Update,
        ]
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ack => write!(f, "ACK"),
            Self::Bye => write!(f, "BYE"),
            Self::Cancel => write!(f, "CANCEL"),
            Self::Info => write!(f, "INFO"),
            Self::Invite => write!(f, "INVITE"),
            Self::Message => write!(f, "MESSAGE"),
            Self::Notify => write!(f, "NOTIFY"),
            Self::Options => write!(f, "OPTIONS"),
            Self::PRack => write!(f, "PRACK"),
            Self::Publish => write!(f, "Publish"),
            Self::Refer => write!(f, "REFER"),
            Self::Register => write!(f, "REGISTER"),
            Self::Subscribe => write!(f, "SUBSCRIBE"),
            Self::Update => write!(f, "UPDATE")
        }
    }
}

pub mod tokenizer {
    use super::Method;
    use crate::{Error, NomError};
    use std::convert::TryInto;

    impl<'a> TryInto<Method> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<Method, Error> {
            use std::str::from_utf8;

            match from_utf8(self.value)? {
                part if part.eq_ignore_ascii_case("ACK") => Ok(Method::Ack),
                part if part.eq_ignore_ascii_case("BYE") => Ok(Method::Bye),
                part if part.eq_ignore_ascii_case("CANCEL") => Ok(Method::Cancel),
                part if part.eq_ignore_ascii_case("INFO") => Ok(Method::Info),
                part if part.eq_ignore_ascii_case("INVITE") => Ok(Method::Invite),
                part if part.eq_ignore_ascii_case("MESSAGE") => Ok(Method::Message),
                part if part.eq_ignore_ascii_case("NOTIFY") => Ok(Method::Notify),
                part if part.eq_ignore_ascii_case("OPTIONS") => Ok(Method::Options),
                part if part.eq_ignore_ascii_case("PRACK") => Ok(Method::PRack),
                part if part.eq_ignore_ascii_case("PUBLISH") => Ok(Method::Publish),
                part if part.eq_ignore_ascii_case("REFER") => Ok(Method::Refer),
                part if part.eq_ignore_ascii_case("REGISTER") => Ok(Method::Register),
                part if part.eq_ignore_ascii_case("SUBSCRIBE") => Ok(Method::Subscribe),
                part if part.eq_ignore_ascii_case("UPDATE") => Ok(Method::Update),
                part => Err(Error::ParseError(format!("Invalid method `{}`", part))),
            }
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
