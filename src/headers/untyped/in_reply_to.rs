use rsip_derives::UntypedHeader;

/// The `In-Reply-To` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct InReplyTo(String);
