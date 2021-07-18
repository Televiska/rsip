use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `From` header in its [untyped](super) form.
#[derive(ToTypedHeader, UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct From(String);
