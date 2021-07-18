use rsip_derives::UntypedHeader;

/// The `Error-Info` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct ErrorInfo(String);
