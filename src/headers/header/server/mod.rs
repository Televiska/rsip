use macros::{DefaultTokenizer, HeaderExtImpl, HeaderNewType};

#[derive(HeaderNewType, DefaultTokenizer, HeaderExtImpl, Debug, PartialEq, Eq, Clone)]
pub struct Server(String);

impl Default for Server {
    fn default() -> Self {
        Self("rsip".into())
    }
}
