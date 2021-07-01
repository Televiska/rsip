use macros::{DefaultTokenizer, HeaderExtImpl, HeaderNewType};

#[derive(HeaderNewType, DefaultTokenizer, HeaderExtImpl, Debug, PartialEq, Eq, Clone)]
#[header(display_name = "Call-ID")]
pub struct CallId(String);
