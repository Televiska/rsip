use macros::{Display, FromIntoInner, FromStr, HasValue};

#[derive(HasValue, Display, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct Method(String);
