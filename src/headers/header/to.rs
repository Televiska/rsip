use crate::headers::{named::Tag, Header, NamedHeader, NamedParam};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct To(pub NamedHeader<NamedParam>);

impl To {
    pub fn tag(&self) -> Option<&Tag> {
        self.0.params.iter().find_map(|param| match param {
            NamedParam::Tag(tag) => Some(tag),
            _ => None,
        })
    }
}

impl From<NamedHeader<NamedParam>> for To {
    fn from(named: NamedHeader<NamedParam>) -> Self {
        Self(named)
    }
}

impl Into<Header> for To {
    fn into(self) -> Header {
        Header::To(self)
    }
}

impl Into<libsip::headers::Header> for To {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::To(Into::<libsip::headers::NamedHeader>::into(self.0))
    }
}

impl std::convert::From<libsip::headers::NamedHeader> for To {
    fn from(from: libsip::headers::NamedHeader) -> Self {
        To(Into::<NamedHeader<NamedParam>>::into(from))
    }
}
