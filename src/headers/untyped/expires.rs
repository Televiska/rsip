use macros::UntypedHeader;

#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Expires(String);

impl Expires {
    pub fn seconds(&self) -> Result<u32, crate::Error> {
        use crate::headers::untyped::UntypedHeader;

        Ok(self.value().parse::<u32>()?)
    }
}

impl From<u32> for Expires {
    fn from(from: u32) -> Self {
        Self(from.to_string())
    }
}
