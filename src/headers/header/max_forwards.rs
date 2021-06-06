use macros::{IntegerTyped, UntypedHeader};

#[derive(UntypedHeader, IntegerTyped, Debug, PartialEq, Eq, Clone)]
#[header(integer_type = "u16")]
pub struct MaxForwards(String);
