use macros::{DefaultTokenizer, HeaderExtImpl, HeaderNewType};

#[derive(HeaderNewType, DefaultTokenizer, HeaderExtImpl, Debug, PartialEq, Eq, Clone)]
pub struct UserAgent(String);

impl Default for UserAgent {
    fn default() -> Self {
        Self("rsip".into())
    }
}
