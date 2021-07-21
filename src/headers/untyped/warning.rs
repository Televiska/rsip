use rsip_derives::UntypedHeader;

/// The `Warning` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Warning(String);
