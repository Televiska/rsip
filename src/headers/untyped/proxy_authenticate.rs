use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `Proxy-Authentication` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct ProxyAuthenticate(String);
