use crate::common;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ContentEncoding(pub common::ContentType);

impl From<libsip::headers::ContentType> for ContentEncoding {
    fn from(from: libsip::headers::ContentType) -> Self {
        Self(from.into())
    }
}

impl Into<libsip::headers::Header> for ContentEncoding {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::ContentEncoding(self.0.into())
    }
}
