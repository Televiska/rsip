use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `Proxy-Authorization` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct ProxyAuthorization(String);
