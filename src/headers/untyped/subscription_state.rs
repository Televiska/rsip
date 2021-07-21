use rsip_derives::UntypedHeader;

/// The `Subscription` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct SubscriptionState(String);
