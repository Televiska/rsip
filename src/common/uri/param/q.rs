use macros::{IntoParam, NewType};

//TODO: add typed + default
#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct Q(String);
