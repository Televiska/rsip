use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `Contact` header in its [untyped](super) form.
#[derive(ToTypedHeader, UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Contact(String);
