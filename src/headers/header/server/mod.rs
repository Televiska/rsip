use macros::{StringTyped, UntypedHeader};

#[derive(UntypedHeader, StringTyped, Debug, PartialEq, Eq, Clone)]
pub struct Server(String);

impl Default for Server {
    fn default() -> Self {
        Self("rsip".into())
    }
}
