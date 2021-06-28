use macros::{StringTyped, UntypedHeader};

#[derive(UntypedHeader, StringTyped, Debug, PartialEq, Eq, Clone)]
#[header(display_name = "Call-ID")]
pub struct CallId(String);
