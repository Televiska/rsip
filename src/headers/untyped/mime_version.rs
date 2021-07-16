use rsip_derives::UntypedHeader;

#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct MimeVersion(String);
