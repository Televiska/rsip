#[doc(hidden)]
pub use super::tokenizers::NameParamsListTokenizer as Tokenizer;

use super::MediaType;
use crate::Error;
use rsip_derives::TypedHeader;
use std::convert::TryFrom;

/// The `Accept` header in its [typed](super) form.
#[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
pub struct Accept(pub Vec<MediaType>);

impl From<Vec<MediaType>> for Accept {
    fn from(media_types: Vec<MediaType>) -> Self {
        Self(media_types)
    }
}

impl<'a> TryFrom<Tokenizer<'a>> for Accept {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        let media_types = tokenizer
            .0
            .into_iter()
            .map(MediaType::try_from)
            .collect::<Result<Vec<MediaType>, Error>>()?;
        Ok(Self(media_types))
    }
}

impl std::fmt::Display for Accept {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
