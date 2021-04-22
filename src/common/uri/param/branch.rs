use macros::{Display, FromIntoInner, FromStr, HasValue};
use uuid::Uuid;

#[derive(HasValue, Display, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct Branch(String);

impl<'a> Default for Branch {
    fn default() -> Self {
        Branch::new(format!("z9hG4bK-televiska-{}", Uuid::new_v4()))
    }
}
