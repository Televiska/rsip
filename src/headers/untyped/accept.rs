use rsip_derives::UntypedHeader;

/// The `Accept` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Accept(String);
