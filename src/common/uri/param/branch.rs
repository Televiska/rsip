use macros::{IntoParam, NewType};

#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct Branch(String);

impl<'a> Default for Branch {
    fn default() -> Self {
        use uuid::Uuid;
        Self::new(format!("z9hG4bK-viska-{}", Uuid::new_v4()))
    }
}
