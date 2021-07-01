use macros::{DefaultTokenizer, HeaderExtImpl, HeaderNewType};

#[derive(HeaderNewType, DefaultTokenizer, HeaderExtImpl, Debug, PartialEq, Eq, Clone)]
//#[header(integer_type = "u32")]
pub struct ContentLength(String);

impl Default for ContentLength {
    fn default() -> Self {
        Self("0".into())
    }
}
