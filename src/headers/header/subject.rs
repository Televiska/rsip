use crate::headers::Header;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Subject(String);

impl Into<String> for Subject {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for Subject {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<Header> for Subject {
    fn into(self) -> Header {
        Header::Subject(self)
    }
}

impl Into<libsip::headers::Header> for Subject {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::Subject(self.into())
    }
}
