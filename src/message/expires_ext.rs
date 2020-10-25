use crate::{
    headers::{ContactParam, Expires, Header, MinExpires},
    Request, Response, SipMessage,
};

pub trait ExpiresExt {
    fn contact_header_expires(&self) -> Option<String>;
    fn expires_header(&self) -> Option<&Expires>;
    fn min_expires_header(&self) -> Option<&MinExpires>;
}

impl ExpiresExt for Request {
    fn contact_header_expires(&self) -> Option<String> {
        match header_opt!(self.headers().iter(), Header::Contact) {
            Some(contact) => contact
                .0
                .params
                .iter()
                .find(|param| match param {
                    ContactParam::Custom(key, _) if key == "expires" => true,
                    _ => false,
                })
                .map(|param| param.value())
                .flatten(),
            None => None,
        }
    }

    fn expires_header(&self) -> Option<&Expires> {
        header_opt!(self.headers().iter(), Header::Expires)
    }

    fn min_expires_header(&self) -> Option<&MinExpires> {
        header_opt!(self.headers().iter(), Header::MinExpires)
    }
}

impl ExpiresExt for Response {
    fn contact_header_expires(&self) -> Option<String> {
        match header_opt!(self.headers().iter(), Header::Contact) {
            Some(contact) => contact
                .0
                .params
                .iter()
                .find(|param| match param {
                    ContactParam::Custom(key, _) if key == "expires" => true,
                    _ => false,
                })
                .map(|param| param.value())
                .flatten(),
            None => None,
        }
    }

    fn expires_header(&self) -> Option<&Expires> {
        header_opt!(self.headers().iter(), Header::Expires)
    }

    fn min_expires_header(&self) -> Option<&MinExpires> {
        header_opt!(self.headers().iter(), Header::MinExpires)
    }
}

impl ExpiresExt for SipMessage {
    fn contact_header_expires(&self) -> Option<String> {
        match header_opt!(self.headers().iter(), Header::Contact) {
            Some(contact) => contact
                .0
                .params
                .iter()
                .find(|param| match param {
                    ContactParam::Custom(key, _) if key == "expires" => true,
                    _ => false,
                })
                .map(|param| param.value())
                .flatten(),
            None => None,
        }
    }

    fn expires_header(&self) -> Option<&Expires> {
        header_opt!(self.headers().iter(), Header::Expires)
    }

    fn min_expires_header(&self) -> Option<&MinExpires> {
        header_opt!(self.headers().iter(), Header::MinExpires)
    }
}
