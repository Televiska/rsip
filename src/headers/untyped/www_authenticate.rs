use rsip_derives::{ToTypedHeader, UntypedHeader};

#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
#[header(display_name = "WWW-Authenticate")]
pub struct WwwAuthenticate(String);
