use crate::{common::Method, headers::untyped::ToTypedHeader, Error};
use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `CSeq` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
#[header(display_name = "CSeq")]
pub struct CSeq(String);

impl CSeq {
    pub fn seq(&self) -> Result<u32, Error> {
        self.typed().map(|s| s.seq)
    }

    pub fn mut_seq(&mut self, new_seq: u32) -> Result<&mut Self, Error> {
        let mut typed = self.typed()?;
        typed.seq = new_seq;
        self.0 = typed.into();
        Ok(self)
    }

    pub fn method(&self) -> Result<Method, Error> {
        self.typed().map(|s| s.method)
    }

    pub fn mut_method(&mut self, new_method: Method) -> Result<&mut Self, Error> {
        let mut typed = self.typed()?;
        typed.method = new_method;
        self.0 = typed.into();
        Ok(self)
    }
}
