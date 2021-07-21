use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `Authorization` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Authorization(String);
