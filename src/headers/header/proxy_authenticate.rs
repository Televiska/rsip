#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProxyAuthenticate(String);

impl Into<String> for ProxyAuthenticate {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for ProxyAuthenticate {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<libsip::headers::Header> for ProxyAuthenticate {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::ProxyAuthenticate(self.into())
    }
}
