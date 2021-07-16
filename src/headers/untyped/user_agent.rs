use macros::UntypedHeader;

#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct UserAgent(String);

impl Default for UserAgent {
    fn default() -> Self {
        UserAgent("rsip".into())
    }
}
