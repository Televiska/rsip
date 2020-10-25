#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProxyRequire(String);

impl Into<String> for ProxyRequire {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for ProxyRequire {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<libsip::headers::Header> for ProxyRequire {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::ProxyRequire(self.into())
    }
}
