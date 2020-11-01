use crate::{
    common::{Method, Uri, Version},
    error::Header as ErrorHeader,
    headers::{self, Header, Headers},
    Error, SipMessage,
};
use bytes::Bytes;
use nom::error::VerboseError;
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

    pub fn authorization_header(&self) -> Result<&headers::Authorization, Error> {
        header!(
            self.headers().iter(),
            Header::Authorization,
            Error::MissingHeader(ErrorHeader::Authorization)
        )
    }
}

impl TryFrom<libsip::core::SipMessage> for Request {
    type Error = &'static str;

    fn try_from(sip_message: libsip::core::SipMessage) -> Result<Self, Self::Error> {
        match sip_message {
            libsip::core::SipMessage::Request {
                method,
                uri,
                version,
                headers,
                body,
            } => Ok(Self {
                method: method.into(),
                uri: uri.into(),
                version: version.into(),
                headers: headers
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<_>>()
                    .into(),
                body,
            }),
            libsip::core::SipMessage::Response { .. } => {
                Err("Can't convert a SipMessage::Response into Request !")
            }
        }
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

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Into::<libsip::core::SipMessage>::into(self.clone())
        )
    }
}

impl Into<libsip::core::SipMessage> for Request {
    fn into(self) -> libsip::core::SipMessage {
        let mut headers = libsip::headers::Headers::new();
        headers.extend(self.headers.into_iter().map(Into::into).collect::<Vec<_>>());
        libsip::core::SipMessage::Request {
            method: self.method.into(),
            uri: self.uri.into(),
            version: self.version.into(),
            headers,
            body: self.body,
        }
    }
}

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
}
