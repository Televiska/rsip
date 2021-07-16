use rsip_derives::{ToTypedHeader, UntypedHeader};

#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Authorization(String);
