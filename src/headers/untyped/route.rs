use rsip_derives::UntypedHeader;

/// The `Route` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Route(String);
