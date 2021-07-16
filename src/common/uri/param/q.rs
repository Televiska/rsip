use rsip_derives::{IntoParam, NewType};

//TODO: add typed + default
#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct Q(String);
