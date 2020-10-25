#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ErrorInfo(String);

impl Into<String> for ErrorInfo {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for ErrorInfo {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<libsip::headers::Header> for ErrorInfo {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::ErrorInfo(self.into())
    }
}
