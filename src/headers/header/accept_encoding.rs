use crate::common;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AcceptEncoding(pub common::ContentType);

impl From<libsip::headers::ContentType> for AcceptEncoding {
    fn from(from: libsip::headers::ContentType) -> Self {
        Self(from.into())
    }
}

impl Into<libsip::headers::Header> for AcceptEncoding {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::AcceptEncoding(self.0.into())
    }
}
