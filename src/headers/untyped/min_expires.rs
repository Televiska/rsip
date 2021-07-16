use rsip_derives::UntypedHeader;

#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct MinExpires(String);

impl MinExpires {
    pub fn seconds(&self) -> Result<u32, crate::Error> {
        use crate::headers::untyped::UntypedHeader;

        Ok(self.value().parse::<u32>()?)
    }
}

impl From<u32> for MinExpires {
    fn from(from: u32) -> Self {
        Self(from.to_string())
    }
}
