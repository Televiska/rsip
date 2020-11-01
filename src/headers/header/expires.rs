use crate::headers::Header;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Expires(pub u32);

impl Default for Expires {
    fn default() -> Self {
        Self(3600)
    }
}

impl Into<u32> for Expires {
    fn into(self) -> u32 {
        self.0
    }
}

impl Into<Header> for Expires {
    fn into(self) -> Header {
        Header::Expires(self)
    }
}

impl Into<libsip::headers::Header> for Expires {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::Expires(self.into())
    }
}

impl From<u32> for Expires {
    fn from(from: u32) -> Self {
        Self(from)
    }
}
