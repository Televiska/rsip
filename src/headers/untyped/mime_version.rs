use rsip_derives::UntypedHeader;

/// The `Mime-Version` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct MimeVersion(String);
