use rsip_derives::UntypedHeader;

/// The `Authentication` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct AuthenticationInfo(String);
