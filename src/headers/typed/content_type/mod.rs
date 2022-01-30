#[doc(hidden)]
pub use super::tokenizers::MediaTypeTokenizer as Tokenizer;

use super::MediaType;
use rsip_derives::TypedHeader;
use std::convert::TryFrom;

/// The `ContentType` header in its [typed](super) form.
#[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
pub struct ContentType(pub MediaType);

impl<'a> TryFrom<Tokenizer<'a>> for ContentType {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        MediaType::try_from(tokenizer).map(Self)
    }
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
