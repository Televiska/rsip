use rsip_derives::{IntoParam, NewType};

/// Simple NewType around String. Intended to be used for the `q` parameter found in the `Contact`
/// header.
//TODO: add typed + default
#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct Q(String);
