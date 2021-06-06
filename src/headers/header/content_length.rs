use macros::{IntegerTyped, UntypedHeader};

#[derive(UntypedHeader, IntegerTyped, Debug, PartialEq, Eq, Clone)]
#[header(integer_type = "u32")]
pub struct ContentLength(String);
