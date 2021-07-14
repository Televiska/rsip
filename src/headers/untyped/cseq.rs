use macros::{ToTypedHeader, UntypedHeader};

#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
#[header(display_name = "CSeq")]
pub struct CSeq(String);
