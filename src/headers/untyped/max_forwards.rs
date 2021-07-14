use macros::UntypedHeader;

#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct MaxForwards(String);

impl Default for MaxForwards {
    fn default() -> Self {
        Self("0".into())
    }
}
