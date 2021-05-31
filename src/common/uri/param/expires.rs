use macros::{FromIntoInner, FromStr, HasValue, ValueDisplay};

//TODO: add typed + default
#[derive(HasValue, ValueDisplay, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct Expires(String);
