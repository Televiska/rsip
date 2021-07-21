use rsip_derives::UntypedHeader;

/// The `Content-Length` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct ContentLength(String);

impl Default for ContentLength {
    fn default() -> Self {
        Self("0".into())
    }
}

impl ContentLength {
    pub fn length(&self) -> Result<u32, crate::Error> {
        use crate::headers::untyped::UntypedHeader;

        Ok(self.value().parse::<u32>()?)
    }
}

impl From<u32> for ContentLength {
    fn from(from: u32) -> Self {
        Self(from.to_string())
    }
}
