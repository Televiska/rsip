use crate::headers::Header;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RetryAfter(String);

impl Into<String> for RetryAfter {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for RetryAfter {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<Header> for RetryAfter {
    fn into(self) -> Header {
        Header::RetryAfter(self)
    }
}

impl Into<libsip::headers::Header> for RetryAfter {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::RetryAfter(self.into())
    }
}
