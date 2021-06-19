use macros::NewType;

//TODO: add typed + default
#[derive(NewType, Debug, PartialEq, Eq, Clone)]
pub struct Expires(String);
