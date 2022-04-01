#[doc(hidden)]
pub use super::tokenizers::UriWithParamsListTokenizer as Tokenizer;

use crate::common::uri::{UriWithParams, UriWithParamsList};
use rsip_derives::TypedHeader;
use std::convert::TryFrom;

/// The `Record-Route` header in its [typed](super) form.
#[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
pub struct RecordRoute(pub UriWithParamsList);

impl RecordRoute {
    pub fn uris(&self) -> &[UriWithParams] {
        self.0.uris()
    }
}

impl From<UriWithParamsList> for RecordRoute {
    fn from(uri_with_params_list: UriWithParamsList) -> Self {
        Self(uri_with_params_list)
    }
}

impl<'a> TryFrom<Tokenizer<'a>> for RecordRoute {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        Ok(Self(UriWithParamsList::try_from(tokenizer)?))
    }
}

impl std::fmt::Display for RecordRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
