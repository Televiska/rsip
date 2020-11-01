use crate::headers::Header;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct XFsSendingMessage(String);

impl Into<String> for XFsSendingMessage {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for XFsSendingMessage {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<Header> for XFsSendingMessage {
    fn into(self) -> Header {
        Header::XFsSendingMessage(self)
    }
}

impl Into<libsip::headers::Header> for XFsSendingMessage {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::XFsSendingMessage(self.into())
    }
}
