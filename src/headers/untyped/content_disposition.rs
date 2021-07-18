use rsip_derives::UntypedHeader;

/// The `Content-Disposal` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct ContentDisposition(String);
