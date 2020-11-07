use super::{NamedParamTrait, NamedParams};
use crate::common::uri::Uri;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NamedHeader<T: NamedParamTrait> {
    pub display_name: Option<String>,
    pub uri: Uri,
    pub params: NamedParams<T>,
}

impl<T: NamedParamTrait> From<Uri> for NamedHeader<T> {
    fn from(uri: Uri) -> Self {
        Self {
            uri,
            display_name: Default::default(),
            params: Default::default(),
        }
    }
}

impl<T: NamedParamTrait> NamedHeader<T> {
    pub fn username(&self) -> Option<String> {
        self.uri.auth.as_ref().map(|a| a.username.clone())
    }

    pub fn uri_mut(&mut self) -> &mut Uri {
        &mut self.uri
    }

    pub fn params_mut(&mut self) -> &mut NamedParams<T> {
        &mut self.params
    }

    pub fn add_param(&mut self, param: T) {
        self.params.push(param)
    }
}

impl<T: NamedParamTrait> Default for NamedHeader<T> {
    fn default() -> Self {
        Self {
            display_name: None,
            uri: Default::default(),
            params: vec![].into(),
        }
    }
}

impl<T: NamedParamTrait> Into<libsip::headers::NamedHeader> for NamedHeader<T> {
    fn into(self) -> libsip::headers::NamedHeader {
        libsip::headers::NamedHeader {
            display_name: self.display_name,
            uri: self.uri.into(),
            parameters: self.params.into(),
        }
    }
}

impl<T: NamedParamTrait> From<libsip::headers::NamedHeader> for NamedHeader<T> {
    fn from(from: libsip::headers::NamedHeader) -> Self {
        Self {
            display_name: from.display_name,
            uri: from.uri.into(),
            params: from.parameters.into(),
        }
    }
}
