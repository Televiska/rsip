use crate::headers::Header;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Timestamp(pub u32);

impl Default for Timestamp {
    fn default() -> Self {
        Self(70)
    }
}

impl Into<u32> for Timestamp {
    fn into(self) -> u32 {
        self.0
    }
}

impl Into<Header> for Timestamp {
    fn into(self) -> Header {
        Header::Timestamp(self)
    }
}

impl Into<libsip::headers::Header> for Timestamp {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::Timestamp(self.into())
    }
}

impl From<u32> for Timestamp {
    fn from(from: u32) -> Self {
        Self(from)
    }
}
