use crate::headers::Header;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProxyAuthorization(String);

impl Into<String> for ProxyAuthorization {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for ProxyAuthorization {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<Header> for ProxyAuthorization {
    fn into(self) -> Header {
        Header::ProxyAuthorization(self)
    }
}

impl Into<libsip::headers::Header> for ProxyAuthorization {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::ProxyAuthorization(self.into())
    }
}
