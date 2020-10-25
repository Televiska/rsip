#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AuthenticationInfo(String);

impl Into<String> for AuthenticationInfo {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for AuthenticationInfo {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<libsip::headers::Header> for AuthenticationInfo {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::AuthenticationInfo(self.into())
    }
}

