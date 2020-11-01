use crate::headers::Header;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ContentDisposition(pub String);

impl Into<String> for ContentDisposition {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for ContentDisposition {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<Header> for ContentDisposition {
    fn into(self) -> Header {
        Header::ContentDisposition(self)
    }
}

impl Into<libsip::headers::Header> for ContentDisposition {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::ContentDisposition(self.into())
    }
}
