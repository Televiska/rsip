use rsip_derives::{ToTypedHeader, UntypedHeader};

#[derive(ToTypedHeader, UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Contact(String);
