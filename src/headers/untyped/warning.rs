use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `Warning` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Warning(String);
