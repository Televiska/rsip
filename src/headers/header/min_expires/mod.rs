use macros::{DefaultTokenizer, HeaderExtImpl, HeaderNewType};

#[derive(HeaderNewType, DefaultTokenizer, HeaderExtImpl, Debug, PartialEq, Eq, Clone)]
//#[header(integer_type = "u32")]
pub struct MinExpires(String);
