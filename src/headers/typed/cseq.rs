#[doc(hidden)]
pub use super::tokenizers::CseqTokenizer as Tokenizer;

use crate::common::Method;
use rsip_derives::TypedHeader;
use std::convert::TryFrom;

/// The `CSeq` header in its [typed](super) form.
#[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
pub struct CSeq {
    pub seq: u16,
    pub method: Method,
}

impl From<(u16, Method)> for CSeq {
    fn from(tuple: (u16, Method)) -> Self {
        Self {
            seq: tuple.0,
            method: tuple.1,
        }
    }
}

impl<'a> TryFrom<Tokenizer<'a>> for CSeq {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        Ok(CSeq {
            seq: tokenizer.seq.parse::<u16>()?,
            method: tokenizer.method.parse::<Method>()?,
        })
    }
}

impl std::fmt::Display for CSeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.seq, self.method)
    }
}
