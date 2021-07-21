use rsip_derives::{IntoParam, NewType};

/// Simple NewType around String. Intended to be used for the `transport` parameter found in the
/// SIP(S) uris.
#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct Transport(String);
