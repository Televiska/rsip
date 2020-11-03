use crate::{
    common::{
        uri::{Branch, HostWithPort, Param, Uri},
        Transport, Version,
    },
    headers::Header,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Via {
    pub version: Version,
    pub transport: Transport,
    pub uri: Uri,
}

impl Via {
    pub fn branch(&self) -> Option<&Branch> {
        self.uri.params.iter().find_map(|param| match param {
            Param::Branch(branch) => Some(branch),
            _ => None,
        })
    }

    pub fn received(&self) -> Option<&HostWithPort> {
        self.uri.params.iter().find_map(|param| match param {
            Param::Received(received) => Some(received),
            _ => None,
        })
    }

    pub fn rport(&self) -> Option<u16> {
        self.uri
            .params
            .iter()
            .find_map(|param| match param {
                Param::RPort(rport) => Some(*rport),
                _ => None,
            })
            .flatten()
    }
}

impl Default for Via {
    fn default() -> Self {
        Self {
            version: Default::default(),
            transport: Default::default(),
            uri: Default::default(),
        }
    }
}

impl From<Uri> for Via {
    fn from(uri: Uri) -> Self {
        Self {
            uri,
            ..Default::default()
        }
    }
}

impl Into<Header> for Via {
    fn into(self) -> Header {
        Header::Via(self)
    }
}

impl Into<libsip::headers::via::ViaHeader> for Via {
    fn into(self) -> libsip::headers::via::ViaHeader {
        libsip::headers::via::ViaHeader {
            version: self.version.into(),
            transport: self.transport.into(),
            uri: self.uri.into(),
        }
    }
}

impl From<libsip::headers::via::ViaHeader> for Via {
    fn from(from: libsip::headers::via::ViaHeader) -> Self {
        Via {
            version: from.version.into(),
            transport: from.transport.into(),
            uri: from.uri.into(),
        }
    }
}

impl Into<libsip::headers::Header> for Via {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::Via(libsip::headers::via::ViaHeader {
            version: self.version.into(),
            transport: self.transport.into(),
            uri: self.uri.into(),
        })
    }
}
