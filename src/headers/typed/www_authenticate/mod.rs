pub mod tokenizer;

pub use tokenizer::Tokenizer;

use crate::common::auth::{Algorithm, Qop};
use crate::{common::auth, Error};
use macros::TypedHeader;
use std::convert::{TryFrom, TryInto};

#[derive(TypedHeader, Eq, PartialEq, Clone, Debug, Default)]
pub struct WwwAuthenticate {
    pub scheme: auth::Scheme,
    pub realm: String,
    pub domain: Option<String>,
    pub nonce: String,
    pub opaque: Option<String>,
    pub stale: Option<String>,
    pub algorithm: Option<Algorithm>,
    //TODO: support multiple Qop
    pub qop: Option<Qop>,
    pub charset: Option<String>,
    //pub userhash: Option<bool>,
}

impl<'a> TryFrom<Tokenizer<'a>> for WwwAuthenticate {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        Ok(WwwAuthenticate {
            scheme: tokenizer.scheme.try_into()?,
            realm: find_param(&tokenizer.params, "realm")
                .ok_or_else(|| Error::InvalidParam("missing realm".into()))?
                .into(),
            domain: find_param(&tokenizer.params, "domain").map(Into::into),
            nonce: find_param(&tokenizer.params, "nonce")
                .ok_or_else(|| Error::InvalidParam("missing nonce".into()))?
                .into(),
            opaque: find_param(&tokenizer.params, "opaque").map(Into::into),
            stale: find_param(&tokenizer.params, "stale").map(Into::into),
            algorithm: find_param(&tokenizer.params, "algorithm")
                .map(TryInto::try_into)
                .transpose()?,
            qop: find_param(&tokenizer.params, "qop")
                .map(TryInto::try_into)
                .transpose()?,
            charset: find_param(&tokenizer.params, "charset").map(Into::into),
        })
    }
}

impl std::fmt::Display for WwwAuthenticate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} realm=\"{}\"", self.scheme, self.realm)?;
        if let Some(domain) = &self.domain {
            write!(f, ", domain=\"{}\"", domain)?;
        }

        write!(f, ", nonce=\"{}\"", self.nonce)?;

        if let Some(opaque) = &self.opaque {
            write!(f, ", opaque=\"{}\"", opaque)?;
        }

        if let Some(stale) = &self.stale {
            write!(f, ", stale={}", stale)?;
        }

        if let Some(algorithm) = &self.algorithm {
            write!(f, ", algorithm={}", algorithm)?;
        }

        if let Some(qop) = &self.qop {
            write!(f, ", qop=\"{}\"", qop)?;
        }

        if let Some(charset) = &self.charset {
            write!(f, ", charset={}", charset)?;
        }

        Ok(())
    }
}

fn find_param<'a>(params: &[(&'a str, &'a str)], name: &str) -> Option<&'a str> {
    params.iter().find_map(|(key, value)| {
        if key.eq_ignore_ascii_case(name) {
            Some(*value)
        } else {
            None
        }
    })
}
