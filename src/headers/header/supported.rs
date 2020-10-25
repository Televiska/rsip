#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Supported(Vec<String>);

impl Into<Vec<String>> for Supported {
    fn into(self) -> Vec<String> {
        self.0
    }
}

impl From<Vec<String>> for Supported {
    fn from(from: Vec<String>) -> Self {
        Self(from)
    }
}

impl Into<libsip::headers::Header> for Supported {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::Supported(self.into())
    }
}
