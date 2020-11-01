use crate::headers::Header;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Organization(String);

impl Into<String> for Organization {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for Organization {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<Header> for Organization {
    fn into(self) -> Header {
        Header::Organization(self)
    }
}

impl Into<libsip::headers::Header> for Organization {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::Organization(self.into())
    }
}
