use rsip_derives::UntypedHeader;

/// The `Server` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Server(String);

impl Default for Server {
    fn default() -> Self {
        Self("rsip".into())
    }
}
