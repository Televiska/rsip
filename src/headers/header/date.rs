use crate::headers::Header;
//
//TODO: this should be datetime!
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Date(pub String);

impl Into<String> for Date {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for Date {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<Header> for Date {
    fn into(self) -> Header {
        Header::Date(self)
    }
}

impl Into<libsip::headers::Header> for Date {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::Date(self.into())
    }
}
