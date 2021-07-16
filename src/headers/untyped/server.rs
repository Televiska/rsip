use rsip_derives::UntypedHeader;

#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Server(String);

impl Default for Server {
    fn default() -> Self {
        Self("rsip".into())
    }
}
