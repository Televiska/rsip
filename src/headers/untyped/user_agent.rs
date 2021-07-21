use rsip_derives::UntypedHeader;

/// The `User-Agent` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct UserAgent(String);

impl Default for UserAgent {
    fn default() -> Self {
        UserAgent("rsip".into())
    }
}
