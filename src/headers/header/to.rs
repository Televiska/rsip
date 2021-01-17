use crate::{
    common::Uri,
    headers::{named::Tag, Header, NamedHeader, NamedParam, NamedParams},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct To(pub NamedHeader<NamedParam>);

impl To {
    pub fn tag(&self) -> Option<&Tag> {
        self.0.params.iter().find_map(|param| match param {
            NamedParam::Tag(tag) => Some(tag),
            _ => None,
        })
    }

    pub fn with_tag(&mut self, tag: impl Into<Tag>) {
        self.0
            .params
            .retain(|param| !matches!(param, NamedParam::Tag(_)));

        self.0.params.push(NamedParam::Tag(tag.into()));
    }

    pub fn with_display_name(mut self, display_name: Option<String>) -> Self {
        self.0.display_name = display_name;
        self
    }

    pub fn with_uri(mut self, uri: Uri) -> Self {
        self.0.uri = uri;
        self
    }

    pub fn with_params(mut self, params: NamedParams<NamedParam>) -> Self {
        self.0.params = params;
        self
    }
}

impl From<NamedHeader<NamedParam>> for To {
    fn from(named: NamedHeader<NamedParam>) -> Self {
        Self(named)
    }
}

impl From<Uri> for To {
    fn from(uri: Uri) -> Self {
        Self(uri.into())
    }
}

impl Into<Header> for To {
    fn into(self) -> Header {
        Header::To(self)
    }
}

impl Into<libsip::headers::Header> for To {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::To(Into::<libsip::headers::NamedHeader>::into(self.0))
    }
}

impl std::convert::From<libsip::headers::NamedHeader> for To {
    fn from(from: libsip::headers::NamedHeader) -> Self {
        To(Into::<NamedHeader<NamedParam>>::into(from))
    }
}
