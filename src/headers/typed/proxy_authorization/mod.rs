#[doc(hidden)]
pub use super::tokenizers::AuthTokenizer as Tokenizer;

use rsip_derives::TypedHeader;
use std::convert::TryFrom;

/// The `Proxy-Authorization` header in its [typed](super) form.
#[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
pub struct ProxyAuthorization(pub super::Authorization);

impl<'a> TryFrom<Tokenizer<'a>> for ProxyAuthorization {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        Ok(ProxyAuthorization(super::Authorization::try_from(
            tokenizer,
        )?))
    }
}

impl std::fmt::Display for ProxyAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
