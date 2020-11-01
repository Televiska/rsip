use crate::headers::Header;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InReplyTo(pub String);

impl Into<String> for InReplyTo {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for InReplyTo {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<Header> for InReplyTo {
    fn into(self) -> Header {
        Header::InReplyTo(self)
    }
}

impl Into<libsip::headers::Header> for InReplyTo {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::InReplyTo(self.into())
    }
}
