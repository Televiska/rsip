use crate::Error;
use rsip_derives::UntypedHeader;

/// The `Expires` header in its [untyped](super) form.
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

impl std::convert::TryFrom<Expires> for u32 {
    type Error = Error;

    fn try_from(from: Expires) -> Result<Self, Self::Error> {
        from.seconds()
    }
}
