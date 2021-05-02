use crate::{
    common::{
        method::{self, Method},
        uri::{self, Uri},
        version::{self, Version},
    },
    headers::{header, Headers},
    Error, NomError, SipMessage,
};
use std::convert::{TryFrom, TryInto};

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

    pub fn parse(tokenizer: Tokenizer) -> Result<Self, Error> {
        Ok(Self {
            method: tokenizer.method.try_into()?,
            uri: tokenizer.uri.try_into()?,
            version: tokenizer.version.try_into()?,
            headers: tokenizer
                .headers
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, Error>>()?
                .into(),
            body: tokenizer.body.into(),
        })
    }
}

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

impl TryFrom<&[u8]> for Request {
    type Error = Error;

    fn try_from(from: &[u8]) -> Result<Self, Self::Error> {
        Self::parse(Tokenizer::tokenize(from)?.1)
    }
}

impl TryFrom<Vec<u8>> for Request {
    type Error = Error;

    fn try_from(from: Vec<u8>) -> Result<Self, Self::Error> {
        Self::parse(Tokenizer::tokenize(&from)?.1)
    }
}

impl TryFrom<&str> for Request {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Self::parse(Tokenizer::tokenize(from.as_bytes())?.1)
    }
}

impl TryFrom<String> for Request {
    type Error = Error;

    fn try_from(from: String) -> Result<Self, Self::Error> {
        Self::parse(Tokenizer::tokenize(&from.as_bytes())?.1)
    }
}

impl<'a> TryFrom<Tokenizer<'a>> for Request {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Error> {
        Self::parse(tokenizer)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tokenizer<'a> {
    pub method: method::Tokenizer<'a>,
    pub uri: uri::Tokenizer<'a>,
    pub version: version::Tokenizer<'a>,
    pub headers: Vec<header::Tokenizer<'a>>,
    pub body: &'a [u8],
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
        use nom::{bytes::complete::tag, multi::many0, sequence::tuple};

        let (rem, (method, uri, version)) = tuple((
            method::Tokenizer::tokenize,
            uri::Tokenizer::tokenize,
            version::Tokenizer::tokenize,
        ))(part)?;
        let (rem, headers) = many0(header::Tokenizer::tokenize)(rem)?;
        let (body, _) = tag("\r\n")(rem)?;

        Ok((
            &[],
            Self {
                method,
                uri,
                version,
                headers,
                body,
            },
        ))
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
