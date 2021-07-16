use macros::{ToTypedHeader, UntypedHeader};

#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct To(String);
