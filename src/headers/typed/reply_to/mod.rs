#[doc(hidden)]
pub use super::tokenizers::UriWithParamsTokenizer as Tokenizer;

use crate::common::uri::UriWithParams;
use rsip_derives::TypedHeader;
use std::convert::TryFrom;

/// The `Record-Route` header in its [typed](super) form.
#[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
pub struct ReplyTo(pub UriWithParams);

impl From<UriWithParams> for ReplyTo {
    fn from(uri_with_params: UriWithParams) -> Self {
        Self(uri_with_params)
    }
}

impl<'a> TryFrom<Tokenizer<'a>> for ReplyTo {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        Ok(Self(UriWithParams::try_from(tokenizer)?))
    }
}

impl std::fmt::Display for ReplyTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
