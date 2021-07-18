use rsip_derives::UntypedHeader;

/// The `Supported` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Supported(String);
