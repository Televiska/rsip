use crate::{
    common::{
        uri::{param, Param},
        Uri,
    },
    headers::untyped::ToTypedHeader,
    Error,
};
use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `Contact` header in its [untyped](super) form.
#[derive(ToTypedHeader, UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Contact(String);

impl Contact {
    pub fn display_name(&self) -> Result<Option<String>, Error> {
        self.typed().map(|s| s.display_name)
    }

    pub fn uri(&self) -> Result<Uri, Error> {
        Ok(self.typed()?.uri)
    }

    pub fn params(&self) -> Result<Vec<Param>, Error> {
        Ok(self.typed()?.params)
    }

    pub fn expires(&self) -> Result<Option<param::Expires>, Error> {
        self.typed().map(|s| s.expires().cloned())
    }

    pub fn with_uri(mut self, uri: Uri) -> Result<Self, Error> {
        let mut typed = self.typed()?;
        typed.uri = uri;
        self.0 = typed.into();
        Ok(self)
    }

    //TODO: this should be replace_uri not mut uri
    pub fn mut_uri(&mut self, uri: Uri) -> Result<&mut Self, Error> {
        let mut typed = self.typed()?;
        typed.uri = uri;
        self.0 = typed.into();
        Ok(self)
    }

    pub fn with_params(mut self, params: Vec<Param>) -> Result<Self, Error> {
        let mut typed = self.typed()?;
        typed.params = params;
        self.0 = typed.into();
        Ok(self)
    }

    pub fn mut_params(&mut self, params: Vec<Param>) -> Result<&mut Self, Error> {
        let mut typed = self.typed()?;
        typed.params = params;
        self.0 = typed.into();
        Ok(self)
    }
}
