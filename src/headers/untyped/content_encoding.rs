use rsip_derives::UntypedHeader;

/// The `Content-Encoding` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct ContentEncoding(String);
