use rsip_derives::UntypedHeader;

/// The `Proxy-Authorization` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct ProxyAuthorization(String);
