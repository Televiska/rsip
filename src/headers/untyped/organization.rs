use rsip_derives::UntypedHeader;

/// The `Organization` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Organization(String);
