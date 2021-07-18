use rsip_derives::UntypedHeader;

/// The `Unsupported` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Unsupported(String);
