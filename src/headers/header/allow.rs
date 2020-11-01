use crate::{common::Method, headers::Header};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Allow(pub Vec<Method>);

impl Into<Vec<Method>> for Allow {
    fn into(self) -> Vec<Method> {
        self.0
    }
}

impl From<Vec<Method>> for Allow {
    fn from(from: Vec<Method>) -> Self {
        Self(from)
    }
}

impl Into<Header> for Allow {
    fn into(self) -> Header {
        Header::Allow(self)
    }
}

impl Into<libsip::headers::Header> for Allow {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::Allow(self.0.into_iter().map(Into::into).collect::<Vec<_>>())
    }
}

impl From<Vec<libsip::core::Method>> for Allow {
    fn from(from: Vec<libsip::core::Method>) -> Self {
        Self(from.into_iter().map(Into::into).collect::<Vec<_>>())
    }
}
