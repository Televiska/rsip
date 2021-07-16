pub use tokenizer::Tokenizer;

use crate::{
    common::{
        method::{self, Method},
        uri::{self, Uri},
        version::{self, Version},
    },
    headers::{header, Headers},
    Error, SipMessage,
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

    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.body
    }
}

impl super::HasHeaders for Request {
    fn headers(&self) -> &Headers {
        &self.headers
    }

    fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}\r\n{}\r\n{}",
            self.method,
            self.uri,
            self.version,
            self.headers,
            String::from_utf8_lossy(&self.body)
        )
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
        Tokenizer::tokenize(from)?.1.try_into()
    }
}

impl TryFrom<Vec<u8>> for Request {
    type Error = Error;

    fn try_from(from: Vec<u8>) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(&from)?.1.try_into()
    }
}

impl TryFrom<&str> for Request {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(from.as_bytes())?.1.try_into()
    }
}

impl TryFrom<String> for Request {
    type Error = Error;

    fn try_from(from: String) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(&from.as_bytes())?.1.try_into()
    }
}

impl TryFrom<bytes::Bytes> for Request {
    type Error = Error;

    fn try_from(from: bytes::Bytes) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(&from)?.1.try_into()
    }
}

impl From<Request> for String {
    fn from(req: Request) -> Self {
        req.to_string()
    }
}

impl From<Request> for Vec<u8> {
    fn from(req: Request) -> Self {
        req.to_string().into_bytes()
    }
}

impl From<Request> for bytes::Bytes {
    fn from(req: Request) -> Self {
        Self::from(req.to_string())
    }
}

pub mod tokenizer {
    use super::{header, method, uri, version, Request};
    use crate::{Error, NomError};
    use std::convert::TryInto;

    impl<'a> TryInto<Request> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<Request, Error> {
            Ok(Request {
                method: self.method.try_into()?,
                uri: self.uri.try_into()?,
                version: self.version.try_into()?,
                headers: self
                    .headers
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, Error>>()?
                    .into(),
                body: self.body.into(),
            })
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Tokenizer<'a> {
        pub method: method::Tokenizer<'a>,
        pub uri: uri::Tokenizer<'a>,
        pub version: version::Tokenizer<'a, &'a [u8]>,
        pub headers: Vec<header::Tokenizer<'a>>,
        pub body: &'a [u8],
    }

    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
            use nom::{bytes::complete::tag, multi::many0, sequence::tuple};

            let (rem, (method, uri, version, _)) = tuple((
                method::Tokenizer::tokenize,
                uri::Tokenizer::tokenize,
                version::Tokenizer::tokenize,
                tag("\r\n"),
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
}
