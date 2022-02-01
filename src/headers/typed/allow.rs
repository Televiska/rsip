use crate::{common::Method, Error};
use rsip_derives::TypedHeader;
use std::convert::TryFrom;

#[doc(hidden)]
pub use super::tokenizers::TokenListTokenizer as Tokenizer;

/// The `Allow` header in its [typed](super) form.
#[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
pub struct Allow(pub Vec<Method>);

impl From<Vec<Method>> for Allow {
    fn from(methods: Vec<Method>) -> Self {
        Self(methods)
    }
}

impl<'a> TryFrom<Tokenizer<'a>> for Allow {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        use std::str::FromStr;

        let methods = tokenizer
            .tokens
            .into_iter()
            .map(Method::from_str)
            .collect::<Result<Vec<Method>, Error>>()?;
        Ok(Self(methods))
    }
}

impl std::fmt::Display for Allow {
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
