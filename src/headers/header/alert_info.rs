use crate::headers::Header;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AlertInfo(String);

impl Into<String> for AlertInfo {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for AlertInfo {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<Header> for AlertInfo {
    fn into(self) -> Header {
        Header::AlertInfo(self)
    }
}

impl Into<libsip::headers::Header> for AlertInfo {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::AlertInfo(self.into())
    }
}
