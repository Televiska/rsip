use rsip_derives::{IntoParam, NewType};

/// Simple NewType around String. Intended to be used for the `maddr` parameter found in the
/// `Via` header.
#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct Maddr(String);
