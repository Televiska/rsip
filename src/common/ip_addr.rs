use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub trait IpAddrExt {
    fn localhost() -> IpAddr;
    fn localhost_v6() -> IpAddr;
}

impl IpAddrExt for IpAddr {
    fn localhost() -> Self {
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
    }

    fn localhost_v6() -> Self {
        IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))
    }
}

pub trait IpAddrLibsipExt {
    fn into_libsip_domain(self) -> libsip::uri::Domain;
}

impl IpAddrLibsipExt for IpAddr {
    fn into_libsip_domain(self) -> libsip::uri::Domain {
        match self {
            IpAddr::V4(ip_addr) => libsip::uri::Domain::Ipv4(ip_addr, None),
            IpAddr::V6(_) => panic!("libsip does not support V4 ip addr"),
        }
    }
}
