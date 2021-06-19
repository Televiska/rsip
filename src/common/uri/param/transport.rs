use macros::NewType;

#[derive(NewType, Debug, PartialEq, Eq, Clone)]
pub struct Transport(String);
