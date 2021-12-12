use rsip_derives::{IntoParam, NewType};

/// Simple NewType around String. Intended to be used for the `ttl` parameter found in the
/// SIP(S) uris and in `Via` header.
#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct Ttl(String);

#[cfg(feature = "test-utils")]
impl testing_utils::Randomize for Ttl {
    fn random() -> Self {
        Self(testing_utils::rand_num_from(1..=100).to_string())
    }
}
