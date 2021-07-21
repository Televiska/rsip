use rsip_derives::UntypedHeader;

/// The `Content-Type` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct ContentType(String);
