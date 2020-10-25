#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RecordRoute(String);

impl Into<String> for RecordRoute {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for RecordRoute {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<libsip::headers::Header> for RecordRoute {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::RecordRoute(self.into())
    }
}
