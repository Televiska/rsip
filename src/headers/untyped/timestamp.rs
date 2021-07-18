use rsip_derives::UntypedHeader;

/// The `Timestamp` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Timestamp(String);
