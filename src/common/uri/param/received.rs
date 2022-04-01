use rsip_derives::{IntoParam, NewType};
use std::net::IpAddr;

/// Simple NewType around String. Intended to be used for the `received` parameter found in the `Via`
/// header.
#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct Received(String);

impl Received {
    pub fn parse(&self) -> Result<IpAddr, std::net::AddrParseError> {
        self.0.parse()
    }
}

#[cfg(feature = "test-utils")]
impl testing_utils::Randomize for Received {
    fn random() -> Self {
        Self(std::net::IpAddr::random().to_string())
    }
}
