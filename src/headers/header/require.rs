#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Require(String);

impl Into<String> for Require {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for Require {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<libsip::headers::Header> for Require {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::Require(self.into())
    }
}
