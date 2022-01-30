#[doc(hidden)]
pub use super::tokenizers::MediaTypeListTokenizer as Tokenizer;

use rsip_derives::TypedHeader;
use std::convert::TryFrom;

/// The `Accept` header in its [typed](super) form.
#[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
pub struct Accept {
    media_types: Vec<MediaType>
}

pub struct MediaType {
    name: String,
    params: Vec<MediaTypeParam>,
    relative_quality: Option<RelativeQuality>
}

impl<'a> TryFrom<Tokenizer<'a>> for Accept {
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
