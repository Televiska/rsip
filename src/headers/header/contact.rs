use crate::headers::{ContactParam, NamedHeader};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Contact(pub NamedHeader<ContactParam>);

impl From<NamedHeader<ContactParam>> for Contact {
    fn from(named: NamedHeader<ContactParam>) -> Self {
        Self(named)
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
