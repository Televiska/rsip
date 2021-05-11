use macros::{FromIntoInner, FromStr, HasValue, ParamDisplay};

#[derive(HasValue, ParamDisplay, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct User(String);
