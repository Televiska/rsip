use macros::{FromIntoInner, FromStr, HasValue, ValueDisplay};

#[derive(HasValue, ValueDisplay, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct Received(String);
