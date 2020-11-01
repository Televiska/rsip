use crate::{common, headers::Header};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ContentType(pub common::ContentType);

impl Into<Header> for ContentType {
    fn into(self) -> Header {
        Header::ContentType(self)
    }
}

impl From<libsip::headers::ContentType> for ContentType {
    fn from(from: libsip::headers::ContentType) -> Self {
        Self(from.into())
    }
}

impl Into<libsip::headers::Header> for ContentType {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::ContentType(self.0.into())
    }
}
