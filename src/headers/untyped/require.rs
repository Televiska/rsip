use rsip_derives::UntypedHeader;

/// The `Require` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Require(String);
