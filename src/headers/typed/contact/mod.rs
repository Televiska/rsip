pub mod tokenizer;

pub use tokenizer::Tokenizer;

use crate::common::{
    uri::{param, Param},
    Uri,
};
use rsip_derives::{TypedHeader, UriAndParamsHelpers};
use std::convert::{TryFrom, TryInto};

#[derive(TypedHeader, UriAndParamsHelpers, Eq, PartialEq, Clone, Debug)]
pub struct Contact {
    pub display_name: Option<String>,
    pub uri: Uri,
    pub params: Vec<Param>,
}

impl Contact {
    pub fn expires(&self) -> Option<&param::Expires> {
        self.params.iter().find_map(|param| match param {
            Param::Expires(expires) => Some(expires),
            _ => None,
        })
    }
}

impl<'a> TryFrom<Tokenizer<'a>> for Contact {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        Ok(Contact {
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

impl std::fmt::Display for Contact {
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

impl std::convert::From<crate::common::Uri> for Contact {
    fn from(uri: crate::common::Uri) -> Self {
        Self {
            display_name: None,
            uri,
            params: Default::default(),
        }
    }
}
