use crate::{
    common::Uri,
    headers::{ContactParam, Header, NamedHeader},
    Error,
};
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Contact(pub NamedHeader<ContactParam>);

impl Contact {
    pub fn expires(&self) -> Result<Option<u32>, Error> {
        self.0
            .params
            .iter()
            .find(|param| matches!(param, ContactParam::Custom(key, _) if key == "expires"))
            .map(|param| param.value())
            .flatten()
            .map(|s| {
                s.parse::<u32>()
                    .map_err(|_| Error::InvalidParam("expire failed to cast to u32".into()))
            })
            .transpose()
    }

    pub fn sip_instance(&self) -> Option<String> {
        self.0
            .params
            .iter()
            .find(|param| matches!(param, ContactParam::Custom(key, _) if key == "+sip.instance"))
            .map(|param| param.value())
            .flatten()
            .map(Into::into)
    }
}

impl From<NamedHeader<ContactParam>> for Contact {
    fn from(named: NamedHeader<ContactParam>) -> Self {
        Self(named)
    }
}

impl From<Uri> for Contact {
    fn from(uri: Uri) -> Self {
        Self(uri.into())
    }
}

impl Into<Header> for Contact {
    fn into(self) -> Header {
        Header::Contact(self)
    }
}

impl Into<libsip::headers::ContactHeader> for Contact {
    fn into(self) -> libsip::headers::ContactHeader {
        libsip::headers::ContactHeader {
            display_name: self.0.display_name,
            uri: self.0.uri.into(),
            parameters: self.0.params.into(),
        }
    }
}

impl From<libsip::headers::ContactHeader> for Contact {
    fn from(libsip_header: libsip::headers::ContactHeader) -> Self {
        Self(NamedHeader {
            display_name: libsip_header.display_name,
            uri: libsip_header.uri.into(),
            params: libsip_header
                .parameters
                .into_iter()
                .map(|(key, value)| {
                    let (key, value): (String, Option<String>) =
                        (key, value.map(Into::<String>::into));

                    Into::<ContactParam>::into((key, value))
                })
                .collect::<Vec<ContactParam>>()
                .into(),
        })
    }
}

impl Into<libsip::headers::Header> for Contact {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::Contact(Into::<libsip::headers::ContactHeader>::into(self))
    }
}

impl TryFrom<String> for Contact {
    type Error = crate::Error;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        use libsip::headers::parse::parse_contact_header;
        use nom::error::VerboseError;

        let libsip_header = parse_contact_header::<VerboseError<&[u8]>>(
            format!("Contact: {}\r\n", string).as_bytes(),
        )?
        .1;
        match libsip_header {
            libsip::headers::Header::Contact(contact) => Ok(contact.into()),
            _ => Err(crate::Error::ParseError(
                "got different libsip header!".into(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_string() {
        //let contact_str = format!("sip:38761902@192.168.1.223:5066;+sip.instance=\"<urn:uuid:1e020c2b-46f6-4867-9d11-65547b8967fa>\"");
        //let contact = Contact::try_from(contact_str);
        use libsip::headers::parse::parse_contact_header;
        use nom::error::VerboseError;
        let contact_str = format!("{}", "Contact: sip:guy@example.com +sip.instance=\"<urn:uuid:1e020c2b-46f6-4867-9d11-65547b8967fa>\"\r\n");
        let res = parse_contact_header::<VerboseError<&[u8]>>(contact_str.as_bytes());

        assert!(res.is_ok());
    }
}
