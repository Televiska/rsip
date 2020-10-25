use crate::{
    common::{StatusCode, Version},
    Headers, SipMessage,
};
use std::convert::TryFrom;
use nom::error::VerboseError;
use bytes::Bytes;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Response {
    pub code: StatusCode,
    pub version: Version,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Response {
    pub fn code(&self) -> &StatusCode {
        &self.code
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
}

impl TryFrom<SipMessage> for Response {
    type Error = &'static str;

    fn try_from(sip_message: crate::SipMessage) -> Result<Self, Self::Error> {
        match sip_message {
            crate::SipMessage::Request(_) => {
                Err("Can't convert a models::SipMessage::Response into Request !")
            }
            crate::SipMessage::Response(response) => Ok(response),
        }
    }
}

impl TryFrom<libsip::core::SipMessage> for Response {
    type Error = &'static str;

    fn try_from(sip_message: libsip::core::SipMessage) -> Result<Self, Self::Error> {
        match sip_message {
            libsip::core::SipMessage::Request { .. } => {
                panic!("Can't convert a SipMessage::Response into Request !")
            }
            libsip::core::SipMessage::Response {
                code,
                version,
                headers,
                body,
            } => Ok(Self {
                code: (code as u16).into(),
                version: version.into(),
                headers: headers
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<_>>()
                    .into(),
                body,
            }),
        }
    }
}

impl Into<libsip::core::SipMessage> for Response {
    fn into(self) -> libsip::core::SipMessage {
        let mut headers = libsip::headers::Headers::new();
        headers.extend(self.headers.into_iter().map(Into::into).collect::<Vec<_>>());
        libsip::core::SipMessage::Response {
            code: Into::<u16>::into(self.code) as u32,
            version: self.version.into(),
            headers,
            body: self.body,
        }
    }
}

impl Into<Bytes> for Response {
    fn into(self) -> Bytes {
        crate::SipMessage::from(self).into()
    }
}

impl TryFrom<Bytes> for Response {
    type Error = String;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        use std::convert::TryInto;

        let (_, sip_message) =
            libsip::parse_message::<VerboseError<&[u8]>>(&bytes.to_vec()).map_err(|e| e.to_string())?;

        Ok(sip_message.try_into()?)
    }
}
