use rsip_derives::{IntoParam, NewType};

/// Simple NewType around String. Intended to be used for the `received` parameter found in the `Via`
/// header.
#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct Received(String);

#[cfg(feature = "test-utils")]
impl testing_utils::Randomize for Received {
    fn random() -> Self {
        Self(std::net::IpAddr::random().to_string())
    }
}
