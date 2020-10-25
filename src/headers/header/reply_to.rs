use crate::headers::{NamedHeader, NamedParam};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReplyTo(pub NamedHeader<NamedParam>);

impl std::convert::From<NamedHeader<NamedParam>> for ReplyTo {
    fn from(named: NamedHeader<NamedParam>) -> Self {
        Self(named)
    }
}

impl Into<libsip::headers::Header> for ReplyTo {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::ReplyTo(Into::<libsip::headers::NamedHeader>::into(self.0))
    }
}

impl std::convert::From<libsip::headers::NamedHeader> for ReplyTo {
    fn from(from: libsip::headers::NamedHeader) -> Self {
        ReplyTo(Into::<NamedHeader<NamedParam>>::into(from))
    }
}
