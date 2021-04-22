use crate::{
    common::{Method, Uri, Version},
    headers::Headers,
    SipMessage,
};
//use bytes::Bytes;
//use nom::error::VerboseError;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Request {
    pub method: Method,
    pub uri: Uri,
    pub version: Version,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Request {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }

    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }

    /*
    pub fn parse(part: &[u8]) -> Result<Self, Error> {
        Ok(Self(String::from_utf8(parse(part)?.to_vec()).map_err(
            |e| Error::Utf8Error(crate::error::Header::Accept, e.to_string()),
        )?))
    }*/
}

/*
fn parse<'a>(part: &'a [u8]) -> Result<&'a [u8], Error> {
    use nom::{
        character::complete::space1,
        sequence::tuple,
        bytes::complete::{tag, take_until},
        sequence::delimited,
    };

    Ok(delimited(tuple((tag("Accept:"), space1)), take_until("\r\n"), tag("\r\n"))(part)?.1)
}*/

impl TryFrom<SipMessage> for Request {
    type Error = &'static str;

    fn try_from(sip_message: crate::SipMessage) -> Result<Self, Self::Error> {
        match sip_message {
            crate::SipMessage::Request(request) => Ok(request),
            crate::SipMessage::Response(_) => {
                Err("Can't convert a models::SipMessage::Response into Request !")
            }
        }
    }
}

/*
impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Into::<libsip::core::SipMessage>::into(self.clone())
        )
    }
}*/

/*
impl TryFrom<Bytes> for Request {
    type Error = String;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        use std::convert::TryInto;

        let (_, sip_message) = libsip::parse_message::<VerboseError<&[u8]>>(&bytes.to_vec())
            .map_err(|e| e.to_string())?;

        Ok(sip_message.try_into()?)
    }
}

impl Into<Bytes> for Request {
    fn into(self) -> Bytes {
        crate::SipMessage::from(self).into()
    }
}*/
