use crate::common::{uri::Domain, SocketAddrExt};
use std::net::{IpAddr, SocketAddr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HostWithPort {
    Domain(Domain),
    SocketAddr(SocketAddr),
    IpAddr(IpAddr),
}

impl HostWithPort {
    pub fn domain(self) -> String {
        match self {
            Self::Domain(domain) => domain.host,
            Self::SocketAddr(socket_addr) => socket_addr.ip().to_string(),
            Self::IpAddr(ip_addr) => ip_addr.to_string(),
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
