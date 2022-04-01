use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `Accept` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Accept(String);
