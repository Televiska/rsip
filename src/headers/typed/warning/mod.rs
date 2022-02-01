#[doc(hidden)]
pub use super::tokenizers::WarningTokenizer as Tokenizer;

use crate::common::Uri;
use rsip_derives::TypedHeader;
use std::convert::{TryFrom, TryInto};

/// The `Record-Warning` header in its [typed](super) form.
#[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
pub struct Warning {
    pub code: u16,
    pub uri: Uri,
    pub text: String,
}

impl<'a> TryFrom<Tokenizer<'a>> for Warning {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        Ok(Self {
            code: tokenizer.code.parse::<u16>()?,
            uri: Uri {
                host_with_port: tokenizer.host.try_into()?,
                ..Default::default()
            },
            text: tokenizer.text.into(),
        })
    }
}

impl std::fmt::Display for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.text.starts_with('"') && self.text.ends_with('"') {
            write!(f, "{} {} {}", self.code, self.uri, self.text)
        } else {
            write!(f, "{} {} \"{}\"", self.code, self.uri, self.text)
        }
    }
}
