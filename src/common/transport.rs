//use crate::common::uri::Param;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Transport {
    Udp,
    Tcp,
}

impl Default for Transport {
    fn default() -> Self {
        Self::Udp
    }
}

impl Into<libsip::Transport> for Transport {
    fn into(self) -> libsip::Transport {
        match self {
            Self::Udp => libsip::Transport::Udp,
            Self::Tcp => libsip::Transport::Tcp,
        }
    }
}

impl From<libsip::Transport> for Transport {
    fn from(from: libsip::Transport) -> Transport {
        match from {
            libsip::Transport::Udp => Self::Udp,
            libsip::Transport::Tcp => Self::Tcp
        }
    }
}

/*
impl Into<Param> for Transport {
    fn into(self) -> Param {
        Param::Transport(self)
    }
}

impl Into<libsip::uri::UriParam> for Transport {
    fn into(self) -> libsip::uri::UriParam {
        libsip::uri::UriParam::Transport(self.into())
    }
}*/
