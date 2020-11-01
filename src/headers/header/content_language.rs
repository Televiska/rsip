use crate::{common::Language, headers::Header};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ContentLanguage(pub Language);

impl From<Language> for ContentLanguage {
    fn from(from: Language) -> Self {
        Self(from)
    }
}

impl Into<Header> for ContentLanguage {
    fn into(self) -> Header {
        Header::ContentLanguage(self)
    }
}

impl From<libsip::headers::Language> for ContentLanguage {
    fn from(from: libsip::headers::Language) -> Self {
        Self(from.into())
    }
}

impl Into<libsip::headers::Header> for ContentLanguage {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::ContentLanguage(self.0.into())
    }
}
