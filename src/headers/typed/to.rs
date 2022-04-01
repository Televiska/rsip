#[doc(hidden)]
pub use super::tokenizers::DisplayUriParamsTokenizer as Tokenizer;

use crate::common::uri::param::Tag;
use crate::common::{uri::Param, Uri};
use rsip_derives::{TypedHeader, UriAndParamsHelpers};
use std::convert::{TryFrom, TryInto};

/// The `To` header in its [typed](super) form.
#[derive(TypedHeader, UriAndParamsHelpers, Eq, PartialEq, Clone, Debug)]
pub struct To {
    pub display_name: Option<String>,
    pub uri: Uri,
    pub params: Vec<Param>,
}

impl<'a> TryFrom<Tokenizer<'a>> for To {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        Ok(To {
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

impl To {
    pub fn tag(&self) -> Option<&Tag> {
        self.params.iter().find_map(|param| match param {
            Param::Tag(tag) => Some(tag),
            _ => None,
        })
    }

    pub fn with_tag(mut self, tag: Tag) -> Self {
        self.params
            .retain(|param| !matches!(param, Param::Tag(Tag { .. })));

        self.params.push(Tag::new(tag).into());
        self
    }
}

impl std::fmt::Display for To {
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
                "<{}>{}",
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

impl std::convert::From<crate::common::Uri> for To {
    fn from(uri: crate::common::Uri) -> Self {
        Self {
            display_name: None,
            uri,
            params: Default::default(),
        }
    }
}
