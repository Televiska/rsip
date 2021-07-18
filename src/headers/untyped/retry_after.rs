use rsip_derives::UntypedHeader;

/// The `Retry-After` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct RetryAfter(String);
