use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `Priority` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Priority(String);
