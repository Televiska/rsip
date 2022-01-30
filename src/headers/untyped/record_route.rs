use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `Record-Route` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct RecordRoute(String);
