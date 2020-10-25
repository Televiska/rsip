#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Priority(String);

impl Into<String> for Priority {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for Priority {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<libsip::headers::Header> for Priority {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::Priority(self.into())
    }
}
