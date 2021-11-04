use rsip_derives::UntypedHeader;

/// The `Max-Forwards` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct MaxForwards(String);

impl Default for MaxForwards {
    fn default() -> Self {
        Self("70".into())
    }
}

impl MaxForwards {
    pub fn num(&self) -> Result<u32, crate::Error> {
        use crate::headers::untyped::UntypedHeader;

        Ok(self.value().parse::<u32>()?)
    }
}

impl From<u32> for MaxForwards {
    fn from(from: u32) -> Self {
        Self(from.to_string())
    }
}
