use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `Error-Info` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct ErrorInfo(String);
