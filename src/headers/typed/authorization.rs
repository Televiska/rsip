#[doc(hidden)]
pub use super::tokenizers::AuthTokenizer as Tokenizer;

use crate::{
    common::Uri,
    headers::auth::{self, Algorithm, AuthQop},
    Error,
};
use rsip_derives::TypedHeader;
use std::convert::{TryFrom, TryInto};

/// The `Authorization` header in its [typed](super) form.
#[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
pub struct Authorization {
    pub scheme: auth::Scheme,
    pub username: String,
    pub realm: String,
    pub nonce: String,
    pub uri: Uri,
    pub response: String,
    pub algorithm: Option<Algorithm>,
    //TODO: support username* here in combination with userhash
    //TODO: this qop is not optional in rfc7616
    //also the cnonce and nc optional depends on qop
    //we should use an enum Qop with cnonce & nc fields instead
    //pub cnonce: Option<String>,
    pub opaque: Option<String>,
    //TODO: support multiple Qop
    pub qop: Option<AuthQop>,
    //TODO: this needs to be a u8
    //pub nc: Option<String>,
    //TODO: enable this in combination with username*
    //pub userhash: Option<bool>,
}

impl<'a> TryFrom<Tokenizer<'a>> for Authorization {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        Ok(Authorization {
            scheme: tokenizer.scheme.try_into()?,
            username: find_param(&tokenizer.params, "username")
                .ok_or_else(|| Error::InvalidParam("missing username".into()))?
                .into(),
            realm: find_param(&tokenizer.params, "realm")
                .ok_or_else(|| Error::InvalidParam("missing realm".into()))?
                .into(),
            nonce: find_param(&tokenizer.params, "nonce")
                .ok_or_else(|| Error::InvalidParam("missing nonce".into()))?
                .into(),
            uri: find_param(&tokenizer.params, "uri")
                .ok_or_else(|| Error::InvalidParam("missing uri".into()))?
                .try_into()?,
            response: find_param(&tokenizer.params, "response")
                .ok_or_else(|| Error::InvalidParam("missing response".into()))?
                .into(),
            algorithm: find_param(&tokenizer.params, "algorithm")
                .map(TryInto::try_into)
                .transpose()?,
            opaque: find_param(&tokenizer.params, "opaque").map(Into::into),
            qop: find_qop(&tokenizer.params)?,
        })
    }
}

impl std::fmt::Display for Authorization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} username=\"{}\", realm=\"{}\", nonce=\"{}\", uri=\"{}\", response=\"{}\"",
            self.scheme, self.username, self.realm, self.nonce, self.uri, self.response
        )?;

        if let Some(algorithm) = &self.algorithm {
            write!(f, ", algorithm={}", algorithm)?;
        }

        if let Some(opaque) = &self.opaque {
            write!(f, ", opaque=\"{}\"", opaque)?;
        }

        if let Some(qop) = &self.qop {
            write!(f, ", {}", qop)?;
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
