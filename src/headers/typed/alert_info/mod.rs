#[doc(hidden)]
pub use super::tokenizers::UriWithParamsListTokenizer as Tokenizer;

use crate::common::uri::UriWithParamsList;
use rsip_derives::TypedHeader;
use std::convert::TryFrom;

/// The `Alert-Info` header in its [typed](super) form.
#[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
pub struct AlertInfo(pub UriWithParamsList);

impl From<UriWithParamsList> for AlertInfo {
    fn from(uri_with_params_list: UriWithParamsList) -> Self {
        Self(uri_with_params_list)
    }
}

impl<'a> TryFrom<Tokenizer<'a>> for AlertInfo {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        Ok(Self(UriWithParamsList::try_from(tokenizer)?))
    }
}

impl std::fmt::Display for AlertInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
