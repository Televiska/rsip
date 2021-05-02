use crate::{Error, NomError};
use macros::{Display, FromIntoInner, FromStr, HasValue};
use nom::error::VerboseError;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HostWithPort {
    pub host: Host,
    pub port: Option<Port>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Host {
    Domain(Domain),
    IpAddr(IpAddr),
}

#[derive(HasValue, FromIntoInner, FromStr, Display, Debug, PartialEq, Eq, Clone)]
pub struct Domain(String);
#[derive(HasValue, FromIntoInner, Display, Debug, PartialEq, Eq, Clone)]
pub struct Port(u16);

impl HostWithPort {
    pub fn parse(tokenizer: Tokenizer) -> Result<Self, Error> {
        use std::str::{from_utf8, FromStr};

        let host = from_utf8(tokenizer.host)?;
        let host = match IpAddr::from_str(host) {
            Ok(ip_addr) => Host::IpAddr(ip_addr),
            Err(_) => Host::Domain(host.into()),
        };

        let port = match tokenizer.port {
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
            bytes::complete::{tag, take_until},
            combinator::rest,
            sequence::tuple,
        };

        let (rem, (host_with_port, _)) = tuple((take_until(" "), tag(" ")))(part)?;
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

impl Default for HostWithPort {
    fn default() -> Self {
        Self {
            host: Host::IpAddr(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            port: None,
        }
    }
}

impl From<IpAddr> for HostWithPort {
    fn from(ip_addr: IpAddr) -> Self {
        Self {
            host: Host::IpAddr(ip_addr),
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

impl From<Domain> for HostWithPort {
    fn from(domain: Domain) -> Self {
        Self {
            host: Host::Domain(domain),
            port: None,
        }
    }
}

//TODO: String should be a dns type for better safety
impl From<&str> for HostWithPort {
    fn from(host: &str) -> Self {
        Self {
            host: Host::Domain(host.into()),
            port: None,
        }
    }
}

impl<T, S> From<(T, Option<S>)> for HostWithPort
where
    T: Into<Domain>,
    S: Into<Port>,
{
    fn from(from: (T, Option<S>)) -> Self {
        Self {
            host: Host::Domain(from.0.into()),
            port: from.1.map(|p| p.into()),
        }
    }
}

/*
impl std::fmt::Display for HostWithPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<libsip::uri::Domain>::into(self.clone()))
    }
}*/
