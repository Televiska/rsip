use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `Call-Info` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct CallInfo(String);
