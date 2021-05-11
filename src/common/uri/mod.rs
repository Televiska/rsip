pub mod auth;
pub mod host_with_port;
pub mod param;
pub mod schema;

pub use tokenizer::Tokenizer;

pub use auth::Auth;
pub use host_with_port::{Domain, Host, HostWithPort, Port};
pub use param::{Branch, Param};
pub use schema::Schema;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Uri {
    pub schema: Option<Schema>,
    pub auth: Option<Auth>,
    pub host_with_port: HostWithPort,
    pub params: Vec<Param>,
    pub headers: crate::Headers,
}

impl Uri {
    pub fn username(&self) -> Option<&str> {
        self.auth.as_ref().map(|auth| auth.username.as_ref())
    }

    pub fn host(&self) -> &Host {
        &self.host_with_port.host
    }

    pub fn port(&self) -> Option<&Port> {
        self.host_with_port.port.as_ref()
    }

    pub fn branch(&self) -> Option<&Branch> {
        self.params.iter().find_map(|param| match param {
            Param::Branch(branch) => Some(branch),
            _ => None,
        })
    }
}

impl Default for Uri {
    fn default() -> Self {
        Self {
            schema: Default::default(),
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
            schema: Default::default(),
            host_with_port,
            auth: None,
            params: Default::default(),
            headers: Default::default(),
        }
    }
}

impl From<std::net::SocketAddr> for Uri {
    fn from(socket_addr: std::net::SocketAddr) -> Self {
        Self {
            schema: Default::default(),
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
            schema: Default::default(),
            host_with_port: ip_addr.into(),
            auth: None,
            params: Default::default(),
            headers: Default::default(),
        }
    }
}

/*
impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<libsip::uri::Uri>::into(self.clone()))
    }
}*/

pub mod tokenizer {
    use super::{auth, host_with_port, schema, Uri};
    use crate::{Error, NomError};
    use std::convert::TryInto;

    impl<'a> TryInto<Uri> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<Uri, Error> {
            Ok(Uri {
                schema: self.schema.map(TryInto::try_into).transpose()?,
                auth: self.auth.map(TryInto::try_into).transpose()?,
                host_with_port: self.host_with_port.try_into()?,
                params: Default::default(),
                headers: Default::default(),
            })
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Tokenizer<'a> {
        pub schema: Option<schema::Tokenizer<'a>>,
        pub auth: Option<auth::Tokenizer<'a>>,
        pub host_with_port: host_with_port::Tokenizer<'a>,
        pub params: Option<Vec<&'a [u8]>>,
        pub headers: Option<Vec<&'a [u8]>>,
    }

    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
            use nom::combinator::opt;

            let (rem, schema) = opt(schema::Tokenizer::tokenize)(part)?;
            let (rem, auth) = opt(auth::Tokenizer::tokenize)(rem)?;
            let (rem, host_with_port) = host_with_port::Tokenizer::tokenize(rem)?;

            Ok((
                rem,
                Self {
                    schema,
                    auth,
                    host_with_port,
                    params: None,
                    headers: None,
                },
            ))
        }
    }
}
