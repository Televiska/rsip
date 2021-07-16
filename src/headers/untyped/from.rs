use macros::{ToTypedHeader, UntypedHeader};

#[derive(ToTypedHeader, UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct From(String);
