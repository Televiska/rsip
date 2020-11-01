use crate::headers::Header;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Server(String);

impl Into<String> for Server {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for Server {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Default for Server {
    fn default() -> Self {
        Self("viska".into())
    }
}

impl Into<Header> for Server {
    fn into(self) -> Header {
        Header::Server(self)
    }
}

impl Into<libsip::headers::Header> for Server {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::Server(self.into())
    }
}
