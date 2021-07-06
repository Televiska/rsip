mod host;
mod port;

pub use tokenizer::Tokenizer;

pub use host::{Domain, Host};
pub use port::Port;

use std::convert::{TryFrom, TryInto};
use std::net::{IpAddr, SocketAddr};
use crate::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HostWithPort {
    pub host: Host,
    pub port: Option<Port>,
}

impl std::fmt::Display for HostWithPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.host, self.port.as_ref()) {
            (host, None) => write!(f, "{}", host),
            (host, Some(port)) => write!(f, "{}:{}", host, port),
        }
    }
}

impl TryFrom<String> for HostWithPort {
    type Error = Error;

    fn try_from(from: String) -> Result<Self, Self::Error> {
        from.as_str().try_into()
    }
}

impl TryFrom<&str> for HostWithPort {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        match from.rsplit_once(":") {
            None => Ok(Host::from(from).into()),
            Some((host, port)) => Ok((
                Host::from(String::from(host)),
                TryInto::<Port>::try_into(port)?,
            )
                .into()),
        }
    }
}

impl Default for HostWithPort {
    fn default() -> Self {
        Self {
            host: Default::default(),
            port: None,
        }
    }
}

impl From<Host> for HostWithPort {
    fn from(host: Host) -> Self {
        Self { host, port: None }
    }
}

impl From<IpAddr> for HostWithPort {
    fn from(from: IpAddr) -> Self {
        Self {
            host: from.into(),
            port: None,
        }
    }
}

impl From<SocketAddr> for HostWithPort {
    fn from(socket_addr: SocketAddr) -> Self {
        Self {
            host: Host::IpAddr(socket_addr.ip()),
            port: Some(socket_addr.port().into()),
        }
    }
}

impl TryInto<SocketAddr> for HostWithPort {
    type Error = Error;

    fn try_into(self) -> Result<SocketAddr, Error> {
        let ip_addr: IpAddr = self.host.try_into()?;

        Ok(SocketAddr::new(ip_addr, self.port.unwrap_or(5060.into()).into()))
    }
}

impl From<Domain> for HostWithPort {
    fn from(domain: Domain) -> Self {
        Self {
            host: Host::Domain(domain),
            port: None,
        }
    }
}

impl<H, P> From<(H, P)> for HostWithPort
where
    H: Into<Host>,
    P: Into<Port>,
{
    fn from(from: (H, P)) -> Self {
        Self {
            host: from.0.into(),
            port: Some(from.1.into()),
        }
    }
}

impl<H, P> From<(H, Option<P>)> for HostWithPort
where
    H: Into<Host>,
    P: Into<Port>,
{
    fn from(from: (H, Option<P>)) -> Self {
        Self {
            host: from.0.into(),
            port: from.1.map(Into::into),
        }
    }
}

pub mod tokenizer {
    use super::{Host, HostWithPort};
    use crate::{Error, NomError};
    use nom::error::VerboseError;
    use std::convert::TryInto;

    impl<'a> TryInto<HostWithPort> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<HostWithPort, Error> {
            use std::net::IpAddr;
            use std::str::{from_utf8, FromStr};

            let host = from_utf8(self.host)?;
            let host = match IpAddr::from_str(host) {
                Ok(ip_addr) => Host::IpAddr(ip_addr),
                Err(_) => Host::Domain(host.into()),
            };

            let port = match self.port {
                Some(port) => Some(from_utf8(port)?.parse::<u16>()?).map(Into::into),
                None => None,
            };

            Ok(HostWithPort { host, port })
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Tokenizer<'a> {
        pub host: &'a [u8],
        pub port: Option<&'a [u8]>,
    }

    impl<'a> From<(&'a [u8], Option<&'a [u8]>)> for Tokenizer<'a> {
        fn from(value: (&'a [u8], Option<&'a [u8]>)) -> Self {
            Self {
                host: value.0,
                port: value.1,
            }
        }
    }

    #[allow(clippy::type_complexity)]
    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
            use nom::{
                bytes::complete::{tag, take_till, take_until},
                combinator::rest,
                sequence::tuple,
            };

            let (rem, host_with_port) = take_till(|c| c == b';' || c == b' ')(part)?;
            let (host, port) =
                match tuple::<_, _, VerboseError<&'a [u8]>, _>((take_until(":"), tag(":"), rest))(
                    host_with_port,
                ) {
                    Ok((_, (host, _, port))) => (host, Some(port)),
                    Err(_) => {
                        let (_, host) = rest(host_with_port)?;
                        (host, None)
                    }
                };

            Ok((rem, (host, port).into()))
        }
    }
}
