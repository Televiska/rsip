use macros::{FromIntoInner, FromStr, HasValue, ValueDisplay};

#[derive(HasValue, ValueDisplay, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct Branch(String);

impl<'a> Default for Branch {
    fn default() -> Self {
        use uuid::Uuid;
        Branch::new(format!("z9hG4bK-televiska-{}", Uuid::new_v4()))
    }
}
