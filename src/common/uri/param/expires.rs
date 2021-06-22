use macros::{NewType, IntoParam};

//TODO: add typed + default
#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct Expires(String);
