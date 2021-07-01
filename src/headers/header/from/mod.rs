mod tokenizer;

use macros::HeaderExtImpl;
pub use tokenizer::Tokenizer;

use crate::common::uri::param::Tag;
use crate::common::{uri::Param, Uri};
use std::convert::{TryFrom, TryInto};

#[derive(HeaderExtImpl, Eq, PartialEq, Clone, Debug)]
pub struct From {
    pub display_name: Option<String>,
    pub uri: Uri,
    pub params: Vec<Param>,
}

impl<'a> TryFrom<Tokenizer<'a>> for From {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        Ok(From {
            display_name: tokenizer.display_name.map(Into::into),
            uri: tokenizer.uri.try_into()?,
            params: tokenizer
                .params
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl From {
    pub fn tag(&self) -> Option<&Tag> {
        self.params.iter().find_map(|param| match param {
            Param::Tag(tag) => Some(tag),
            _ => None,
        })
    }
}

impl std::fmt::Display for From {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.display_name {
            Some(display_name) => write!(
                f,
                "{} <{}>{}",
                display_name,
                self.uri,
                self.params
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ),
            None => write!(
                f,
                "{}{}",
                self.uri,
                self.params
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ),
        }
    }
}
