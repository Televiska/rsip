use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `Via` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Via(String);
