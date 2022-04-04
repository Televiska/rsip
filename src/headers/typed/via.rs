#[doc(hidden)]
pub use super::tokenizers::ViaTokenizer as Tokenizer;

use crate::{
    common::{
        uri::{self, param::Branch, Param, Uri},
        Transport, Version,
    },
    Error,
};
use rsip_derives::{TypedHeader, UriAndParamsHelpers};
use std::{
    convert::{TryFrom, TryInto},
    net::IpAddr,
};

/// The `Via` header in its [typed](super) form.
#[derive(TypedHeader, UriAndParamsHelpers, Eq, PartialEq, Clone, Debug)]
pub struct Via {
    pub version: Version,
    pub transport: Transport,
    //TODO: rename to sent-by ?
    pub uri: Uri,
    pub params: Vec<uri::Param>,
}

impl Via {
    pub fn branch(&self) -> Result<&Branch, Error> {
        self.params
            .iter()
            .find_map(|param| match param {
                Param::Branch(branch) => Some(branch),
                _ => None,
            })
            .ok_or_else(|| Error::missing_param("branch"))
    }

    pub fn received(&self) -> Result<Option<IpAddr>, std::net::AddrParseError> {
        self.params
            .iter()
            .find_map(|param| match param {
                Param::Received(received) => Some(received.parse()),
                _ => None,
            })
            .transpose()
    }

    pub fn sent_by(&self) -> &Uri {
        &self.uri
    }

    pub fn sent_protocol(&self) -> &Transport {
        &self.transport
    }
}

impl<'a> TryFrom<Tokenizer<'a>> for Via {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        Ok(Via {
            version: tokenizer.version.try_into()?,
            transport: tokenizer.transport.try_into()?,
            uri: tokenizer.uri.try_into()?,
            params: tokenizer
                .params
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl std::fmt::Display for Via {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{} {}{}",
            self.version,
            self.transport,
            self.uri,
            self.params
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

impl std::convert::From<crate::common::Uri> for Via {
    fn from(uri: crate::common::Uri) -> Self {
        Self {
            version: Default::default(),
            transport: Default::default(),
            uri,
            params: vec![Param::Branch(Default::default())],
        }
    }
}
