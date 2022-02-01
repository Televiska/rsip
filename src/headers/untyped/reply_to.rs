use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `Reply-To` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct ReplyTo(String);
