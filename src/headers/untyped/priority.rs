use rsip_derives::UntypedHeader;

/// The `Priority` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Priority(String);
