use crate::headers::Header;
use macros::{FromIntoInner, FromStr, HasValue, IntoHeader};

#[derive(HasValue, IntoHeader, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct WwwAuthenticate(String);

impl std::fmt::Display for WwwAuthenticate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WWW-Authenticate: {}", self.value())
    }
}
