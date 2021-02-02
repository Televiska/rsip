use crate::headers::Header;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Accept(pub String);

impl Into<String> for Accept {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for Accept {
    fn from(from: String) -> Self {
        Self(from)
    }
}

impl Into<Header> for Accept {
    fn into(self) -> Header {
        Header::Accept(self)
    }
}

impl Into<libsip::headers::Header> for Accept {
    fn into(self) -> libsip::headers::Header {
        panic!("fix libsip accept first, it shouldn't take a Vec<Method>")
    }
}
