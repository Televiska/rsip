use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `Content-Type` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct ContentType(String);
