use crate::Error;
use rsip_derives::NewType;
use std::convert::TryInto;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Host {
    Domain(Domain),
    IpAddr(IpAddr),
}

impl Default for Host {
    fn default() -> Self {
        Self::IpAddr(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
    }
}

impl std::str::FromStr for Host {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<IpAddr>() {
            Ok(ip_addr) => Ok(Host::IpAddr(ip_addr)),
            Err(_) => Ok(Host::Domain(s.into())),
        }
    }
}

#[derive(NewType, Debug, PartialEq, Eq, Clone)]
pub struct Domain(String);

impl std::fmt::Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Host::Domain(domain) => write!(f, "{}", domain),
            Host::IpAddr(ip_addr) => write!(f, "{}", ip_addr),
        }
    }
}

impl From<String> for Host {
    fn from(from: String) -> Self {
        from.as_str().into()
    }
}

impl From<&str> for Host {
    fn from(from: &str) -> Self {
        match from.parse::<IpAddr>() {
            Ok(ip_addr) => Host::IpAddr(ip_addr),
            Err(_) => Host::Domain(from.into()),
        }
    }
}

impl From<IpAddr> for Host {
    fn from(from: IpAddr) -> Self {
        Host::IpAddr(from)
    }
}

impl From<Domain> for Host {
    fn from(from: Domain) -> Self {
        Host::Domain(from)
    }
}

impl TryInto<IpAddr> for Host {
    type Error = Error;

    fn try_into(self) -> Result<IpAddr, Error> {
        match self {
            Self::Domain(_) => Err(Error::Unexpected("cannot convert Host to IpAddr".into())),
            Self::IpAddr(ip_addr) => Ok(ip_addr),
        }
    }
}

impl TryInto<SocketAddr> for Host {
    type Error = Error;

    fn try_into(self) -> Result<SocketAddr, Error> {
        let ip_addr: IpAddr = self.try_into()?;
        Ok(SocketAddr::new(ip_addr, 5060))
    }
}
