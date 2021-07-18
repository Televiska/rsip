use rsip_derives::UntypedHeader;

/// The `Content-Language` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct ContentLanguage(String);
