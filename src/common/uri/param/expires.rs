use macros::{IntoParam, NewType};

//TODO: add typed + default
#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct Expires(String);

impl Expires {
    pub fn seconds(&self) -> Result<u32, crate::Error> {
        Ok(self.value().parse::<u32>()?)
    }
}
