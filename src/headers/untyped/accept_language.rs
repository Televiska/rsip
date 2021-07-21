use rsip_derives::UntypedHeader;

/// The `Accept-Language` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct AcceptLanguage(String);
