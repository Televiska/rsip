use macros::{StringTyped, UntypedHeader};

#[derive(UntypedHeader, StringTyped, Debug, PartialEq, Eq, Clone)]
pub struct UserAgent(String);

impl Default for UserAgent {
    fn default() -> Self {
        Self("rsip".into())
    }
}
