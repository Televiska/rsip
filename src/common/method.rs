use crate::{Error, NomError};
use std::convert::TryFrom;

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

    pub fn parse(tokenizer: Tokenizer) -> Result<Self, Error> {
        use std::str::from_utf8;

        match from_utf8(tokenizer.value)? {
            part if part.eq_ignore_ascii_case("ACK") => Ok(Self::Ack),
            part if part.eq_ignore_ascii_case("BYE") => Ok(Self::Bye),
            part if part.eq_ignore_ascii_case("CANCEL") => Ok(Self::Cancel),
            part if part.eq_ignore_ascii_case("INFO") => Ok(Self::Info),
            part if part.eq_ignore_ascii_case("INVITE") => Ok(Self::Invite),
            part if part.eq_ignore_ascii_case("MESSAGE") => Ok(Self::Message),
            part if part.eq_ignore_ascii_case("NOTIFY") => Ok(Self::Notify),
            part if part.eq_ignore_ascii_case("OPTIONS") => Ok(Self::Options),
            part if part.eq_ignore_ascii_case("PRACK") => Ok(Self::PRack),
            part if part.eq_ignore_ascii_case("PUBLISH") => Ok(Self::Publish),
            part if part.eq_ignore_ascii_case("REFER") => Ok(Self::Refer),
            part if part.eq_ignore_ascii_case("REGISTER") => Ok(Self::Register),
            part if part.eq_ignore_ascii_case("SUBSCRIBE") => Ok(Self::Subscribe),
            part if part.eq_ignore_ascii_case("UPDATE") => Ok(Self::Update),
            part => Err(Error::ParseError(format!("Invalid method `{}`", part))),
        }
    }
}

impl<'a> TryFrom<Tokenizer<'a>> for Method {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Error> {
        Self::parse(tokenizer)
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
        use crate::parser_utils::{create_error_for, opt_sp};
        use nom::{bytes::complete::take_until, sequence::tuple};

        let (rem, (method, _)) = tuple((take_until(" "), opt_sp))(part)?;
        //TODO: helpful to return early in case we parse a response but maybe it should be checked
        //here though
        if method.starts_with(b"SIP/") {
            return Err(create_error_for(method, "SIP version found instead"));
        }

        Ok((rem, method.into()))
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
