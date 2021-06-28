use macros::{IntoParam, NewType};

#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct User(String);
