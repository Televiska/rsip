#[doc(hidden)]
pub use super::tokenizers::AuthTokenizer as Tokenizer;

use rsip_derives::TypedHeader;
use std::convert::TryFrom;

/// The `Proxy-Authenticate` header in its [typed](super) form.
#[derive(TypedHeader, Eq, PartialEq, Clone, Debug, Default)]
pub struct ProxyAuthenticate(pub super::WwwAuthenticate);

impl<'a> TryFrom<Tokenizer<'a>> for ProxyAuthenticate {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        Ok(ProxyAuthenticate(super::WwwAuthenticate::try_from(
            tokenizer,
        )?))
    }
}

impl std::fmt::Display for ProxyAuthenticate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
