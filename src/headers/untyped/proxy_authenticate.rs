use rsip_derives::UntypedHeader;

/// The `Proxy-Authentication` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct ProxyAuthenticate(String);
