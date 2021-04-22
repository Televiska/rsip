use crate::headers::Header;
use macros::{Display, FromIntoInner, FromStr, HasValue, IntoHeader};

#[derive(HasValue, Display, IntoHeader, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct SubscriptionState(String);
