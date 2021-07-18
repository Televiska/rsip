use rsip_derives::UntypedHeader;

/// The `Subject` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Subject(String);
