use macros::{NewType, IntoParam};

#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct Transport(String);
