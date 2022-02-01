#[doc(hidden)]
pub use super::tokenizers::TokenListTokenizer as Tokenizer;

use rsip_derives::TypedHeader;
use std::convert::TryFrom;

/// The `In-Reply-To` header in its [typed](super) form.
#[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
pub struct InReplyTo(pub Vec<String>);

impl<'a> TryFrom<Tokenizer<'a>> for InReplyTo {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        Ok(Self(
            tokenizer
                .tokens
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        ))
    }
}

impl std::fmt::Display for InReplyTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}
