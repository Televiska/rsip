use crate::{
    error::Header as ErrorHeader,
    headers::{ContactParam, Expires, Header, MinExpires},
    Error, Request, Response, SipMessage,
};

pub trait ExpiresExt {
    fn contact_header_expires(&self) -> Result<Option<u32>, Error>;
    fn expires_header(&self) -> Result<&Expires, Error>;
    fn min_expires_header(&self) -> Result<&MinExpires, Error>;
}

impl ExpiresExt for Request {
    fn contact_header_expires(&self) -> Result<Option<u32>, Error> {
        match header_opt!(self.headers().iter(), Header::Contact) {
            Some(contact) => contact.expires(),
            None => Ok(None),
        }
    }

    fn expires_header(&self) -> Result<&Expires, Error> {
        header!(
            self.headers().iter(),
            Header::Expires,
            Error::MissingHeader(ErrorHeader::Expires)
        )
    }

    fn min_expires_header(&self) -> Result<&MinExpires, Error> {
        header!(
            self.headers().iter(),
            Header::MinExpires,
            Error::MissingHeader(ErrorHeader::MinExpires)
        )
    }
}

impl ExpiresExt for Response {
    fn contact_header_expires(&self) -> Result<Option<u32>, Error> {
        match header_opt!(self.headers().iter(), Header::Contact) {
            Some(contact) => contact
                .0
                .params
                .iter()
                .find(|param| matches!(param, ContactParam::Custom(key, _) if key == "expires"))
                .map(|param| param.value())
                .flatten()
                .map(|s| {
                    s.parse::<u32>()
                        .map_err(|_| Error::InvalidParam("expire failed to cast to u32".into()))
                })
                .transpose(),
            None => Ok(None),
        }
    }

    fn expires_header(&self) -> Result<&Expires, Error> {
        header!(
            self.headers().iter(),
            Header::Expires,
            Error::MissingHeader(ErrorHeader::Expires)
        )
    }

    fn min_expires_header(&self) -> Result<&MinExpires, Error> {
        header!(
            self.headers().iter(),
            Header::MinExpires,
            Error::MissingHeader(ErrorHeader::MinExpires)
        )
    }
}

impl ExpiresExt for SipMessage {
    fn contact_header_expires(&self) -> Result<Option<u32>, Error> {
        match self {
            Self::Request(request) => request.contact_header_expires(),
            Self::Response(response) => response.contact_header_expires(),
        }
    }

    fn expires_header(&self) -> Result<&Expires, Error> {
        match self {
            Self::Request(request) => request.expires_header(),
            Self::Response(response) => response.expires_header(),
        }
    }

    fn min_expires_header(&self) -> Result<&MinExpires, Error> {
        match self {
            Self::Request(request) => request.min_expires_header(),
            Self::Response(response) => response.min_expires_header(),
        }
    }
}
