use crate::headers::Header;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallInfo(pub String);

impl Into<String> for CallInfo {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for CallInfo {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<Header> for CallInfo {
    fn into(self) -> Header {
        Header::CallInfo(self)
    }
}

impl Into<libsip::headers::Header> for CallInfo {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::CallInfo(self.into())
    }
}
