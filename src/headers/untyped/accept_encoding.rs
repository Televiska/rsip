use rsip_derives::UntypedHeader;

/// The `Accept-Encoding` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct AcceptEncoding(String);
