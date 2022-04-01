use crate::{
    common::{param::Param, Transport, Uri, Version},
    headers::untyped::ToTypedHeader,
    Error,
};
use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `Via` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Via(String);

impl Via {
    pub fn version(&self) -> Result<Version, Error> {
        self.typed().map(|s| s.version)
    }

    pub fn trasnport(&self) -> Result<Transport, Error> {
        self.typed().map(|s| s.transport)
    }

    pub fn uri(&self) -> Result<Uri, Error> {
        self.typed().map(|s| s.uri)
    }

    pub fn params(&self) -> Result<Vec<Param>, Error> {
        self.typed().map(|s| s.params)
    }

    pub fn with_uri(mut self, uri: Uri) -> Result<Self, Error> {
        self.0 = self.typed()?.with_uri(uri).into();
        Ok(self)
    }

    pub fn mut_uri(&mut self, uri: Uri) -> Result<&mut Self, Error> {
        self.0 = self.typed()?.with_uri(uri).into();
        Ok(self)
    }
}
