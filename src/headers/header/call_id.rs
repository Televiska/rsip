use crate::headers::Header;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallId(pub String);

//TODO: domain should be compiled-configured
//although RFC says that call id is byte-to-byte compared
impl Default for CallId {
    fn default() -> Self {
        Self(format!("{}@example.com", Uuid::new_v4().to_string()))
    }
}

impl Into<String> for CallId {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for CallId {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<Header> for CallId {
    fn into(self) -> Header {
        Header::CallId(self)
    }
}

impl Into<libsip::headers::Header> for CallId {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::CallId(self.into())
    }
}
