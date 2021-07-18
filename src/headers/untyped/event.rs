use rsip_derives::UntypedHeader;

/// The `Event` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Event(String);
