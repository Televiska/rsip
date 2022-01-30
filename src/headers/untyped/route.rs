use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `Route` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Route(String);
