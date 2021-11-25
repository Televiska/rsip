use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `From` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct From(String);
