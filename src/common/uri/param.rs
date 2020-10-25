use crate::common::{uri::HostWithPort, Transport};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Param {
    Transport(Transport),
    Branch(Branch),
    Received(HostWithPort),
    RPort(Option<u16>),
    Other(String, Option<String>),
}

impl Into<libsip::uri::UriParam> for Param {
    fn into(self) -> libsip::uri::UriParam {
        match self {
            Self::Transport(transport) => libsip::uri::UriParam::Transport(transport.into()),
            Self::Branch(branch) => libsip::uri::UriParam::Branch(branch.into()),
            Self::Received(domain) => libsip::uri::UriParam::Received(domain.into()),
            Self::RPort(port) => libsip::uri::UriParam::RPort(port),
            Self::Other(key, value) => libsip::uri::UriParam::Other(key, value),
        }
    }
}

impl From<libsip::uri::UriParam> for Param {
    fn from(from: libsip::uri::UriParam) -> Self {
        match from {
            libsip::uri::UriParam::Transport(transport) => Self::Transport(transport.into()),
            libsip::uri::UriParam::Branch(branch) => Self::Branch(branch.into()),
            libsip::uri::UriParam::Received(received) => Self::Received(received.into()),
            _ => panic!(""),
        }
    }
}

//TODO: should RFC2543 as well
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Branch(String);
impl Default for Branch {
    fn default() -> Self {
        Branch(format!("z9hG4bK-televiska-{}", Uuid::new_v4()))
    }
}

impl Into<String> for Branch {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for Branch {
    fn from(from: String) -> Self {
        Self(from)
    }
}

/*
impl Into<Param> for Branch {
    fn into(self) -> Param {
        Param::Branch(self)
    }
}

impl Into<libsip::uri::UriParam> for Branch {
    fn into(self) -> libsip::uri::UriParam {
        libsip::uri::UriParam::Branch(self.into())
    }
}*/
