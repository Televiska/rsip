pub mod auth;
pub mod host_with_port;
pub mod param;
pub mod scheme;

#[doc(hidden)]
pub use tokenizer::Tokenizer;

pub use auth::Auth;
pub use host_with_port::{Domain, Host, HostWithPort, Port};
pub use param::Param;
pub use scheme::Scheme;

use crate::Transport;
use std::convert::{TryFrom, TryInto};

/// A very flexible SIP(S) URI.
///
/// Note that during parsing, if no port is set, it is returned as `None`. Usually when no port
/// is specified then port 5060 is assumed. But rsip is not acting smart here and delegates that
/// responsibility to you because you might want 5061 (TLS) as default etc.
/// Similarly on generation, if no port is specified, no port is set at all in the final string.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Uri {
    pub scheme: Option<Scheme>,
    pub auth: Option<Auth>,
    pub host_with_port: HostWithPort,
    pub params: Vec<Param>,
    pub headers: crate::Headers,
}

impl Uri {
    pub fn user(&self) -> Option<&str> {
        self.auth.as_ref().map(|auth| auth.user.as_ref())
    }

    pub fn host(&self) -> &Host {
        &self.host_with_port.host
    }

    pub fn port(&self) -> Option<&Port> {
        self.host_with_port.port.as_ref()
    }

    pub fn transport(&self) -> Option<&Transport> {
        self.params.iter().find_map(|param| match param {
            Param::Transport(transport) => Some(transport),
            _ => None,
        })
    }
}

//TODO: improve impl here to remove clones
impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scheme = match &self.scheme {
            Some(scheme) => format!("{}:", scheme),
            None => format!(""),
        };
        let auth = match &self.auth {
            Some(auth) => format!("{}@", auth),
            None => format!(""),
        };

        write!(
            f,
            "{}{}{}{}",
            scheme,
            auth,
            self.host_with_port,
            self.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

impl TryFrom<String> for Uri {
    type Error = crate::Error;

    fn try_from(from: String) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(from.as_bytes())?.1.try_into()
    }
}

impl TryFrom<&str> for Uri {
    type Error = crate::Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Tokenizer::tokenize(from.as_bytes())?.1.try_into()
    }
}

impl Default for Uri {
    fn default() -> Self {
        Self {
            scheme: Default::default(),
            host_with_port: Default::default(),
            auth: None,
            params: Default::default(),
            headers: Default::default(),
        }
    }
}

impl From<HostWithPort> for Uri {
    fn from(host_with_port: HostWithPort) -> Self {
        Self {
            scheme: Default::default(),
            host_with_port,
            auth: None,
            params: Default::default(),
            headers: Default::default(),
        }
    }
}

impl From<(Scheme, Host)> for Uri {
    fn from(tuple: (Scheme, Host)) -> Self {
        Self {
            scheme: Some(tuple.0),
            host_with_port: tuple.1.into(),
            auth: None,
            params: Default::default(),
            headers: Default::default(),
        }
    }
}

impl<H, P> From<(H, P)> for Uri
where
    H: Into<Host>,
    P: Into<Port>,
{
    fn from(tuple: (H, P)) -> Self {
        Self {
            scheme: None,
            host_with_port: (tuple.0.into(), tuple.1.into()).into(),
            auth: None,
            params: Default::default(),
            headers: Default::default(),
        }
    }
}

impl From<Host> for Uri {
    fn from(host: Host) -> Self {
        Self {
            scheme: None,
            host_with_port: host.into(),
            auth: None,
            params: Default::default(),
            headers: Default::default(),
        }
    }
}

impl From<std::net::SocketAddr> for Uri {
    fn from(socket_addr: std::net::SocketAddr) -> Self {
        Self {
            scheme: Default::default(),
            host_with_port: socket_addr.into(),
            auth: None,
            params: Default::default(),
            headers: Default::default(),
        }
    }
}

impl From<std::net::IpAddr> for Uri {
    fn from(ip_addr: std::net::IpAddr) -> Self {
        Self {
            scheme: Default::default(),
            host_with_port: ip_addr.into(),
            auth: None,
            params: Default::default(),
            headers: Default::default(),
        }
    }
}

#[doc(hidden)]
pub mod tokenizer {
    use super::{auth, host_with_port, param, scheme, Uri};
    use crate::{Error, NomError};
    use std::convert::TryInto;

    impl<'a> TryInto<Uri> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<Uri, Error> {
            Ok(Uri {
                scheme: self.scheme.map(TryInto::try_into).transpose()?,
                auth: self.auth.map(TryInto::try_into).transpose()?,
                host_with_port: self.host_with_port.try_into()?,
                params: self
                    .params
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()?,
                headers: Default::default(),
            })
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Tokenizer<'a> {
        pub scheme: Option<scheme::Tokenizer<'a>>,
        pub auth: Option<auth::Tokenizer<'a>>,
        pub host_with_port: host_with_port::Tokenizer<'a>,
        pub params: Vec<param::Tokenizer<'a>>,
        //TODO: why option here?
        pub headers: Option<Vec<&'a [u8]>>,
    }

    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
            use nom::{character::complete::space0, combinator::opt, multi::many0};

            let (rem, scheme) = opt(scheme::Tokenizer::tokenize)(part)?;
            let (rem, auth) = opt(auth::Tokenizer::tokenize)(rem)?;
            let (rem, host_with_port) = host_with_port::Tokenizer::tokenize(rem)?;
            let (rem, params) = many0(param::Tokenizer::tokenize)(rem)?;
            //TODO: remove these smart moves
            let (rem, _) = opt(space0)(rem)?;

            Ok((
                rem,
                Self {
                    scheme,
                    auth,
                    host_with_port,
                    params,
                    //TODO: support headers in the uri
                    headers: None,
                },
            ))
        }

        pub fn tokenize_without_params(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
            use nom::combinator::opt;

            let (rem, scheme) = opt(scheme::Tokenizer::tokenize)(part)?;
            let (rem, auth) = opt(auth::Tokenizer::tokenize)(rem)?;
            let (rem, host_with_port) = host_with_port::Tokenizer::tokenize(rem)?;

            Ok((
                rem,
                Self {
                    scheme,
                    auth,
                    host_with_port,
                    params: vec![],
                    headers: None,
                },
            ))
        }
    }
}
