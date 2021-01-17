use crate::common::{uri::Domain, SocketAddrExt};
use std::net::{IpAddr, SocketAddr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HostWithPort {
    Domain(Domain),
    SocketAddr(SocketAddr),
    IpAddr(IpAddr),
}

pub enum DomainType {
    Domain(String),
    Ip(IpAddr),
}

impl std::fmt::Display for DomainType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ip(ip_addr) => write!(f, "{}", ip_addr),
            Self::Domain(host) => write!(f, "{}", host),
        }
    }
}

impl HostWithPort {
    pub fn domain(self) -> DomainType {
        match self {
            Self::Domain(domain) => DomainType::Domain(domain.host),
            Self::SocketAddr(socket_addr) => DomainType::Ip(socket_addr.ip()),
            Self::IpAddr(ip_addr) => DomainType::Ip(ip_addr),
        }
    }

    pub fn port(self) -> u16 {
        match self {
            Self::Domain(domain) => domain.port.unwrap_or(5060),
            Self::SocketAddr(socket_addr) => socket_addr.port(),
            Self::IpAddr(_) => 5060,
        }
    }
}

impl Default for HostWithPort {
    fn default() -> Self {
        Self::SocketAddr(SocketAddr::localhost(5060))
    }
}

impl From<IpAddr> for HostWithPort {
    fn from(ip_addr: IpAddr) -> Self {
        Self::IpAddr(ip_addr)
    }
}

impl From<SocketAddr> for HostWithPort {
    fn from(socket_addr: SocketAddr) -> Self {
        Self::SocketAddr(socket_addr)
    }
}

impl From<Domain> for HostWithPort {
    fn from(domain: Domain) -> Self {
        Self::Domain(domain)
    }
}

//TODO: String should be a dns type for better safety
impl From<String> for HostWithPort {
    fn from(host: String) -> Self {
        Self::Domain(Domain { host, port: None })
    }
}

impl std::fmt::Display for HostWithPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<libsip::uri::Domain>::into(self.clone()))
    }
}

impl Into<libsip::uri::Domain> for HostWithPort {
    fn into(self) -> libsip::uri::Domain {
        use crate::common::IpAddrLibsipExt;
        use crate::common::SocketAddrLibsipExt;

        match self {
            Self::Domain(domain) => domain.into(),
            Self::SocketAddr(socket_addr) => socket_addr.into_libsip_domain(),
            Self::IpAddr(ip_addr) => ip_addr.into_libsip_domain(),
        }
    }
}

impl From<libsip::uri::Domain> for HostWithPort {
    fn from(from: libsip::uri::Domain) -> Self {
        match from {
            libsip::uri::Domain::Ipv4(ip_addr, port) => Self::SocketAddr(SocketAddr::new(
                std::net::IpAddr::V4(ip_addr),
                port.unwrap_or(5060),
            )),
            libsip::uri::Domain::Domain(domain, port) => Self::Domain((domain, port).into()),
        }
    }
}
