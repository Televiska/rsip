#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UserAgent(String);

//TODO: this should compiled-configured
impl Default for UserAgent {
    fn default() -> Self {
        UserAgent("televiska".into())
    }
}

impl Into<String> for UserAgent {
    fn into(self) -> String {
        self.0
    }
}

impl Into<libsip::headers::Header> for UserAgent {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::UserAgent(self.into())
    }
}

impl From<String> for UserAgent {
    fn from(from: String) -> Self {
        Self(from)
    }
}
