use crate::{common::Language, headers::Header};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AcceptLanguage(pub Language);

impl Into<Header> for AcceptLanguage {
    fn into(self) -> Header {
        Header::AcceptLanguage(self)
    }
}

impl From<Language> for AcceptLanguage {
    fn from(from: Language) -> Self {
        Self(from)
    }
}

impl From<libsip::headers::Language> for AcceptLanguage {
    fn from(from: libsip::headers::Language) -> Self {
        Self(from.into())
    }
}

impl Into<libsip::headers::Header> for AcceptLanguage {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::AcceptLanguage(self.0.into())
    }
}
