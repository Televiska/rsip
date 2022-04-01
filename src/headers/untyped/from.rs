use crate::{
    common::uri::{param::Tag, Param, Uri},
    headers::untyped::ToTypedHeader,
    Error,
};
use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `From` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct From(String);

impl From {
    pub fn display_name(&self) -> Result<Option<String>, Error> {
        self.typed().map(|s| s.display_name)
    }

    pub fn uri(&self) -> Result<Uri, Error> {
        self.typed().map(|s| s.uri)
    }

    pub fn params(&self) -> Result<Vec<Param>, Error> {
        self.typed().map(|s| s.params)
    }

    pub fn tag(&self) -> Result<Option<Tag>, Error> {
        self.typed().map(|s| s.tag().cloned())
    }

    pub fn with_tag(mut self, tag: Tag) -> Result<Self, Error> {
        self.0 = self.typed()?.with_tag(tag).into();
        Ok(self)
    }

    pub fn mut_tag(&mut self, tag: Tag) -> Result<&mut Self, Error> {
        self.0 = self.typed()?.with_tag(tag).into();
        Ok(self)
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
