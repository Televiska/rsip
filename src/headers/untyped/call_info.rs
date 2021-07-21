use rsip_derives::UntypedHeader;

/// The `Call-Info` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct CallInfo(String);
