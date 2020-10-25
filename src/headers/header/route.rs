#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Route(String);

impl Into<String> for Route {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for Route {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<libsip::headers::Header> for Route {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::Route(self.into())
    }
}
