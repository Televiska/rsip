use crate::headers::{NamedHeader, NamedParam};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct From(pub NamedHeader<NamedParam>);

impl std::convert::From<NamedHeader<NamedParam>> for From {
    fn from(named: NamedHeader<NamedParam>) -> Self {
        Self(named)
    }
}

impl Into<libsip::headers::Header> for From {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::From(Into::<libsip::headers::NamedHeader>::into(self.0))
    }
}

impl std::convert::From<libsip::headers::NamedHeader> for From {
    fn from(from: libsip::headers::NamedHeader) -> Self {
        From(Into::<NamedHeader<NamedParam>>::into(from))
    }
}
