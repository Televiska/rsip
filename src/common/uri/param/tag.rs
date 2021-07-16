use rsip_derives::{IntoParam, NewType};

#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct Tag(String);

impl<'a> Default for Tag {
    fn default() -> Self {
        use uuid::Uuid;
        Self::new(format!("viska-{}", Uuid::new_v4()))
    }
}
