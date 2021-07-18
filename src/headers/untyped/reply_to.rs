use rsip_derives::UntypedHeader;

/// The `Reply-To` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct ReplyTo(String);
