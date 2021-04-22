use crate::{common::Version, Headers, Request, Response};

#[derive(Debug, Clone)]
pub enum SipMessage {
    Request(Request),
    Response(Response),
}

impl SipMessage {
    pub fn is_request(&self) -> bool {
        matches!(self, Self::Request(_))
    }

    pub fn is_response(&self) -> bool {
        matches!(self, Self::Response(_))
    }

    pub fn version(&self) -> &Version {
        match self {
            Self::Request(request) => request.version(),
            Self::Response(response) => response.version(),
        }
    }

    pub fn headers(&self) -> &Headers {
        match self {
            Self::Request(request) => request.headers(),
            Self::Response(response) => response.headers(),
        }
    }

    pub fn headers_mut(&mut self) -> &mut Headers {
        match self {
            Self::Request(request) => request.headers_mut(),
            Self::Response(response) => response.headers_mut(),
        }
    }

    pub fn body(&self) -> &Vec<u8> {
        match self {
            Self::Request(request) => request.body(),
            Self::Response(response) => response.body(),
        }
    }

    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        match self {
            Self::Request(request) => request.body_mut(),
            Self::Response(response) => response.body_mut(),
        }
    }
}

/*
impl TryFrom<Bytes> for SipMessage {
    type Error = String;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        let (_, libsip_sip_message) = libsip::parse_message::<VerboseError<&[u8]>>(&bytes.to_vec())
            .map_err(|e| format!("could not convert to models::SipMessage from bytes: {}", e))?;

        Ok(libsip_sip_message.try_into()?)
    }
}

impl Into<Bytes> for SipMessage {
    fn into(self) -> Bytes {
        match self {
            SipMessage::Request(request) => {
                Bytes::from(Into::<libsip::SipMessage>::into(request).to_string())
            }
            SipMessage::Response(response) => {
                Bytes::from(Into::<libsip::SipMessage>::into(response).to_string())
            }
        }
    }
}

impl TryFrom<String> for SipMessage {
    type Error = String;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        let (_, libsip_sip_message) =
            libsip::parse_message::<VerboseError<&[u8]>>(string.as_bytes()).map_err(|e| {
                format!("could not convert to models::SipMessage from string: {}", e)
            })?;

        Ok(libsip_sip_message.try_into()?)
    }
}

impl Into<String> for SipMessage {
    fn into(self) -> String {
        match self {
            SipMessage::Request(request) => Into::<libsip::SipMessage>::into(request).to_string(),
            SipMessage::Response(response) => {
                Into::<libsip::SipMessage>::into(response).to_string()
            }
        }
    }
}

impl TryFrom<Vec<u8>> for SipMessage {
    type Error = String;

    fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        let (_, libsip_sip_message) = libsip::parse_message::<VerboseError<&[u8]>>(&vec)
            .map_err(|e| format!("could not convert to models::SipMessage from vec: {}", e))?;

        Ok(libsip_sip_message.try_into()?)
    }
}

impl Into<Vec<u8>> for SipMessage {
    fn into(self) -> Vec<u8> {
        match self {
            SipMessage::Request(request) => Into::<libsip::SipMessage>::into(request)
                .to_string()
                .into_bytes(),
            SipMessage::Response(response) => Into::<libsip::SipMessage>::into(response)
                .to_string()
                .into_bytes(),
        }
    }
}

impl TryFrom<&str> for SipMessage {
    type Error = String;

    fn try_from(slice: &str) -> Result<Self, Self::Error> {
        let (_, libsip_sip_message) =
            libsip::parse_message::<VerboseError<&[u8]>>(slice.as_bytes())
                .map_err(|e| format!("could not convert to models::SipMessage from vec: {}", e))?;

        Ok(libsip_sip_message.try_into()?)
    }
}

impl From<Request> for SipMessage {
    fn from(request: Request) -> Self {
        SipMessage::Request(request)
    }
}

impl From<Response> for SipMessage {
    fn from(response: Response) -> Self {
        SipMessage::Response(response)
    }
}*/
