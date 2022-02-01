#[doc(hidden)]
pub use super::tokenizers::NameValueTokenizer as Tokenizer;

use crate::{headers::auth::AuthQop, Error};
use rsip_derives::TypedHeader;
use std::convert::TryFrom;

/// The `Authentication-Info` header in its [typed](super) form.
#[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
pub struct AuthenticationInfo {
    pub nextnonce: String,
    pub qop: Option<AuthQop>,
    pub rspauth: Option<String>,
}

impl<'a> TryFrom<Tokenizer<'a>> for AuthenticationInfo {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        Ok(AuthenticationInfo {
            nextnonce: find_param(&tokenizer.params, "nextnonce")
                .ok_or_else(|| Error::InvalidParam("missing nextnonce".into()))?
                .into(),
            qop: find_qop(&tokenizer.params)?,
            rspauth: find_param(&tokenizer.params, "rspauth").map(Into::into),
        })
    }
}

impl std::fmt::Display for AuthenticationInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "nextnonce=\"{}\"", self.nextnonce)?;

        if let Some(rspauth) = &self.rspauth {
            write!(f, ", rspauth=\"{}\"", rspauth)?;
        }

        if let Some(qop) = &self.qop {
            write!(f, ", {}", qop)?;
        }

        Ok(())
    }
}

//TODO: Move to common trait and impl on relevant tokenizers
fn find_param<'a>(params: &[(&'a str, &'a str)], name: &str) -> Option<&'a str> {
    params.iter().find_map(|(key, value)| {
        if key.eq_ignore_ascii_case(name) {
            Some(*value)
        } else {
            None
        }
    })
}

fn find_qop<'a>(params: &[(&'a str, &'a str)]) -> Result<Option<AuthQop>, Error> {
    Ok(match find_param(params, "qop") {
        Some(qop) if qop.eq_ignore_ascii_case("auth") => Some(AuthQop::Auth {
            cnonce: find_param(params, "cnonce")
                .ok_or_else(|| Error::InvalidParam("Found qop, but missing cnonce".into()))?
                .into(),
            nc: find_param(params, "nc")
                .ok_or_else(|| Error::InvalidParam("Found qop, but missing nc".into()))?
                .parse::<u8>()?,
        }),
        Some(qop) if qop.eq_ignore_ascii_case("auth-int") => Some(AuthQop::AuthInt {
            cnonce: find_param(params, "cnonce")
                .ok_or_else(|| Error::InvalidParam("Found qop, but missing cnonce".into()))?
                .into(),
            nc: find_param(params, "nc")
                .ok_or_else(|| Error::InvalidParam("Found qop, but missing nc".into()))?
                .parse::<u8>()?,
        }),
        Some(qop) => return Err(Error::InvalidParam(format!("Found unknown qop: {}", qop))),
        None => None,
    })
}
