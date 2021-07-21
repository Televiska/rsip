use rsip_derives::UntypedHeader;

/// The `Alert-Info` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct AlertInfo(String);
