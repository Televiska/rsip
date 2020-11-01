use crate::headers::Header;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MinExpires(pub u32);

impl Default for MinExpires {
    fn default() -> Self {
        Self(3600)
    }
}

impl Into<u32> for MinExpires {
    fn into(self) -> u32 {
        self.0
    }
}

impl Into<Header> for MinExpires {
    fn into(self) -> Header {
        Header::MinExpires(self)
    }
}

impl Into<libsip::headers::Header> for MinExpires {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::MinExpires(self.into())
    }
}

impl From<u32> for MinExpires {
    fn from(from: u32) -> Self {
        Self(from)
    }
}
