use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `To` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct To(String);
