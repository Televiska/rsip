use crate::{common::Uri, headers::{named::Tag, Header, NamedHeader, NamedParam}};

//TODO: maybe NamedHeader could become a trait instead
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct From(pub NamedHeader<NamedParam>);

impl From {
    pub fn tag(&self) -> Option<&Tag> {
        self.0.params.iter().find_map(|param| match param {
            NamedParam::Tag(tag) => Some(tag),
            _ => None,
        })
    }
}

impl std::convert::From<Uri> for From {
    fn from(uri: Uri) -> Self {
        Self(uri.into())
    }
}

impl std::convert::From<NamedHeader<NamedParam>> for From {
    fn from(named: NamedHeader<NamedParam>) -> Self {
        Self(named)
    }
}

impl Into<Header> for From {
    fn into(self) -> Header {
        Header::From(self)
    }
}

impl Into<libsip::headers::Header> for From {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::From(Into::<libsip::headers::NamedHeader>::into(self.0))
    }
}

impl std::convert::From<libsip::headers::NamedHeader> for From {
    fn from(from: libsip::headers::NamedHeader) -> Self {
        From(Into::<NamedHeader<NamedParam>>::into(from))
    }
}
