#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Unsupported(String);

impl Into<String> for Unsupported {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for Unsupported {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<libsip::headers::Header> for Unsupported {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::Unsupported(self.into())
    }
}
