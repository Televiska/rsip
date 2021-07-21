use rsip_derives::UntypedHeader;

/// The `Record-Route` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct RecordRoute(String);
