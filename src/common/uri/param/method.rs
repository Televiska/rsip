use rsip_derives::{IntoParam, NewType};

#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct Method(String);
