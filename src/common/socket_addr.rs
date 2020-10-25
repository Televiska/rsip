use std::net::{IpAddr, SocketAddr};

pub trait SocketAddrExt {
    fn localhost(port: u16) -> SocketAddr;
    fn localhost_v6(port: u16) -> SocketAddr;
    fn from_tuple(tuple: (IpAddr, u16)) -> SocketAddr;
}

pub trait SocketAddrLibsipExt {
    fn into_libsip_domain(self) -> libsip::uri::Domain;
}

impl SocketAddrExt for SocketAddr {
    fn localhost(port: u16) -> Self {
        use crate::common::IpAddrExt;
        SocketAddr::new(IpAddr::localhost(), port)
    }

    fn localhost_v6(port: u16) -> Self {
        use crate::common::IpAddrExt;
        SocketAddr::new(IpAddr::localhost_v6(), port)
    }

    fn from_tuple(tuple: (IpAddr, u16)) -> Self {
        SocketAddr::new(tuple.0, tuple.1)
    }
}

impl SocketAddrLibsipExt for SocketAddr {
    fn into_libsip_domain(self) -> libsip::uri::Domain {
        let port = self.port();
        match self.ip() {
            IpAddr::V4(ip_addr) => libsip::uri::Domain::Ipv4(ip_addr, Some(port)),
            IpAddr::V6(_) => panic!("libsip does not support V4 ip addr"),
        }
    }
}
