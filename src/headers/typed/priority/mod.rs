#[doc(hidden)]
pub use super::tokenizers::ValueTokenizer as Tokenizer;

use rsip_derives::TypedHeader;
use std::convert::TryFrom;

/// The `Priority` header in its [typed](super) form.
#[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
pub enum Priority {
    NonUrgent,
    Normal,
    Urgent,
    Emergency,
    Other(String),
}

impl<'a> TryFrom<Tokenizer<'a>> for Priority {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        match tokenizer.value {
            s if s.eq_ignore_ascii_case("non-urgent") => Ok(Self::NonUrgent),
            s if s.eq_ignore_ascii_case("normal") => Ok(Self::Normal),
            s if s.eq_ignore_ascii_case("urgent") => Ok(Self::Urgent),
            s if s.eq_ignore_ascii_case("emergency") => Ok(Self::Emergency),
            s => Ok(Self::Other(s.into())),
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonUrgent => write!(f, "non-urgent"),
            Self::Normal => write!(f, "normal"),
            Self::Urgent => write!(f, "urgent"),
            Self::Emergency => write!(f, "emergency"),
            Self::Other(other) => write!(f, "{}", other),
        }
    }
}
