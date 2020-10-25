#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MaxForwards(pub u32);

impl Default for MaxForwards {
    fn default() -> Self {
        Self(70)
    }
}

impl Into<u32> for MaxForwards {
    fn into(self) -> u32 {
        self.0
    }
}

impl Into<libsip::headers::Header> for MaxForwards {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::MaxForwards(self.into())
    }
}

impl From<u32> for MaxForwards {
    fn from(from: u32) -> Self {
        Self(from)
    }
}
