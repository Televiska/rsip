use rsip_derives::UntypedHeader;

/// The `Date` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Date(String);
