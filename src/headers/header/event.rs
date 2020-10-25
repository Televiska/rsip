#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Event {
    Custom(String),
}

impl Into<String> for Event {
    fn into(self) -> String {
        match self {
            Self::Custom(inner) => inner,
        }
    }
}

impl From<String> for Event {
    fn from(from: String) -> Self {
        Self::Custom(from)
    }
}

impl Into<libsip::headers::Header> for Event {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::Event(self.into())
    }
}
