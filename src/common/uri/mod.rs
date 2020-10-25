mod auth;
mod domain;
mod host_with_port;
mod param;
mod schema;

pub use auth::Auth;
pub use domain::Domain;
pub use host_with_port::HostWithPort;
pub use param::{Branch, Param};
pub use schema::Schema;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Uri {
    pub schema: Schema,
    pub host_with_port: HostWithPort,
    pub auth: Option<Auth>,
    pub params: Vec<Param>,
}

/*
pub trait TestsUriExt {
    fn localhost() -> Uri;
    fn localhost_with_port(port: u16) -> Uri;
}

impl TestsUriExt for Uri {
    fn localhost() -> Self {
        use std::net::{IpAddr, Ipv4Addr};

        Self {
            host_with_port: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)).into(),
            ..Default::default()
        }
    }

    fn localhost_with_port(port: u16) -> Self {
        use crate::common::SocketAddrExt;
        use std::net::SocketAddr;

        Self {
            host_with_port: SocketAddr::localhost(port).into(),
            ..Default::default()
        }
    }
}*/

impl Default for Uri {
    fn default() -> Self {
        Self {
            schema: Default::default(),
            host_with_port: Default::default(),
            auth: Default::default(),
            params: Default::default(),
        }
    }
}

impl Into<libsip::uri::Uri> for Uri {
    fn into(self) -> libsip::uri::Uri {
        libsip::uri::Uri {
            schema: Some(self.schema.into()),
            host: self.host_with_port.into(),
            auth: self.auth.map(|a| a.into()),
            parameters: self
                .params
                .into_iter()
                .map(|p| p.into())
                .collect::<Vec<_>>(),
        }
    }
}

impl From<libsip::uri::Uri> for Uri {
    fn from(from: libsip::uri::Uri) -> Self {
        Self {
            schema: from.schema.map(|s| s.into()).unwrap_or_default(),
            host_with_port: from.host.into(),
            auth: from.auth.map(|a| a.into()),
            params: from
                .parameters
                .into_iter()
                .map(|p| p.into())
                .collect::<Vec<_>>(),
        }
    }
}
