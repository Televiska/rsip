use rsip_derives::UntypedHeader;

/// The `Proxy-Require` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct ProxyRequire(String);
