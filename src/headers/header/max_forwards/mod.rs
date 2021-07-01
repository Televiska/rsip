use macros::{DefaultTokenizer, HeaderExtImpl, HeaderNewType};

#[derive(HeaderNewType, DefaultTokenizer, HeaderExtImpl, Debug, PartialEq, Eq, Clone)]
//#[header(integer_type = "u16")]
pub struct MaxForwards(String);

impl Default for MaxForwards {
    fn default() -> Self {
        Self("0".into())
    }
}
