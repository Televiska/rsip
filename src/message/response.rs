#[doc(hidden)]
pub use tokenizer::Tokenizer;

use crate::{
    common::{
        status_code::{self, StatusCode},
        version::{self, Version},
    },
    headers::{header, Headers},
    Error, SipMessage,
};
use std::convert::{TryFrom, TryInto};

/// Response reprsents a SIP response message, which contains a [StatusCode](crate::StatusCode),
/// a [Version](crate::Version), [Headers](crate::Headers) and a
/// body, represented as a `Vec<u8>`, thus not checked for UTF-8 compliance.
///
/// A Response can easily be transformed to a [SipMessage](crate::SipMessage).
/// Also it can be converted to a `String`, `&str`, or `Bytes`, all using the underlying `Debug`
/// trait.
///
/// In order to access specific [headers](crate::headers::untyped), you should take a look on the
/// [HeadersExt](crate::message::HeadersExt) trait that is automatically implemented for any type
/// that has implemented the [HasHeaders](crate::message::HasHeaders) trait, which Response
/// implements it.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Response {
    pub status_code: StatusCode,
    pub version: Version,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Response {
    pub fn status_code(&self) -> &StatusCode {
        &self.status_code
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

impl super::HasHeaders for Response {
    fn headers(&self) -> &Headers {
        &self.headers
    }

    fn headers_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}\r\n{}\r\n{}",
            self.version,
            self.status_code,
            self.headers,
            String::from_utf8_lossy(&self.body)
        )
    }
}

impl TryFrom<SipMessage> for Response {
    type Error = crate::Error;

    fn try_from(sip_message: crate::SipMessage) -> Result<Self, Self::Error> {
        match sip_message {
            crate::SipMessage::Request(_) => Err(crate::Error::Unexpected(
                "Can't convert a models::SipMessage::Response into Request !".into(),
            )),
            crate::SipMessage::Response(response) => Ok(response),
        }
    }
}

impl TryFrom<&[u8]> for Response {
    type Error = Error;

    fn try_from(from: &[u8]) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(from)?.1.try_into()
    }
}

impl TryFrom<Vec<u8>> for Response {
    type Error = Error;

    fn try_from(from: Vec<u8>) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(&from)?.1.try_into()
    }
}

impl TryFrom<&str> for Response {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(from.as_bytes())?.1.try_into()
    }
}

impl TryFrom<String> for Response {
    type Error = Error;

    fn try_from(from: String) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(from.as_bytes())?.1.try_into()
    }
}

impl TryFrom<bytes::Bytes> for Response {
    type Error = Error;

    fn try_from(from: bytes::Bytes) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(&from)?.1.try_into()
    }
}

impl From<Response> for String {
    fn from(res: Response) -> Self {
        res.to_string()
    }
}

impl From<Response> for Vec<u8> {
    fn from(res: Response) -> Self {
        res.to_string().into_bytes()
    }
}

impl From<Response> for bytes::Bytes {
    fn from(res: Response) -> Self {
        bytes::Bytes::from(res.to_string())
    }
}

#[doc(hidden)]
pub mod tokenizer {
    use super::{header, status_code, version, Response};
    use crate::{Error, NomError};
    use std::convert::TryInto;

    impl<'a> TryInto<Response> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<Response, Error> {
            Ok(Response {
                version: self.version.try_into()?,
                status_code: self.status_code.try_into()?,
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
        pub version: version::Tokenizer<'a, &'a [u8]>,
        pub status_code: status_code::Tokenizer<'a, &'a [u8]>,
        pub headers: Vec<header::Tokenizer<'a>>,
        pub body: &'a [u8],
    }

    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
            use nom::{
                branch::alt,
                bytes::complete::{tag, take_until},
                character::complete::space1,
                multi::many0,
                sequence::tuple,
            };

            let (rem, (version, _, status_code, _)) = tuple((
                version::Tokenizer::tokenize,
                space1,
                status_code::Tokenizer::tokenize,
                tag("\r\n"),
            ))(part)?;

            let (body, (headers, _)) = alt((
                tuple((take_until("\r\n\r\n"), tag("\r\n\r\n"))),
                tuple((take_until("\r\n"), tag("\r\n"))),
            ))(rem)?;
            let (_, headers) = many0(header::Tokenizer::tokenize)(headers)?;

            Ok((
                &[],
                Self {
                    version,
                    status_code,
                    headers,
                    body,
                },
            ))
        }
    }
}
