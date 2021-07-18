use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `WWW-Authenticate` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
#[header(display_name = "WWW-Authenticate")]
pub struct WwwAuthenticate(String);
