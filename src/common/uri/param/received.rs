use macros::NewType;

#[derive(NewType, Debug, PartialEq, Eq, Clone)]
pub struct Received(String);
