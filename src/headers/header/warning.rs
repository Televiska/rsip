use crate::headers::Header;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Warning(String);

impl Into<String> for Warning {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for Warning {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<Header> for Warning {
    fn into(self) -> Header {
        Header::Warning(self)
    }
}

impl Into<libsip::headers::Header> for Warning {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::Warning(self.into())
    }
}
