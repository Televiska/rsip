use crate::headers::Header;

//TODO: make all inner fields as public
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UserAgent(pub String);

//TODO: this should compiled-configured
impl Default for UserAgent {
    fn default() -> Self {
        UserAgent("rsip".into())
    }
}

impl Into<String> for UserAgent {
    fn into(self) -> String {
        self.0
    }
}

impl Into<Header> for UserAgent {
    fn into(self) -> Header {
        Header::UserAgent(self)
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
