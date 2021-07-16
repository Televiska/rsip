pub mod auth;
pub mod host_with_port;
pub mod param;
pub mod schema;

pub use tokenizer::Tokenizer;

pub use auth::Auth;
pub use host_with_port::{Domain, Host, HostWithPort, Port};
pub use param::Param;
pub use schema::Schema;

use std::convert::{TryFrom, TryInto};

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
}

//TODO: improve impl here to remove clones
impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let schema = match &self.schema {
            Some(schema) => format!("{}:", schema),
            None => format!(""),
        };
        let auth = match &self.auth {
            Some(auth) => format!("{}@", auth),
            None => format!(""),
        };

        write!(
            f,
            "{}{}{}{}",
            schema,
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

impl From<(Schema, Host)> for Uri {
    fn from(tuple: (Schema, Host)) -> Self {
        Self {
            schema: Some(tuple.0),
            host_with_port: tuple.1.into(),
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

pub mod tokenizer {
    use super::{auth, host_with_port, param, schema, Uri};
    use crate::{Error, NomError};
    use std::convert::TryInto;

    impl<'a> TryInto<Uri> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<Uri, Error> {
            Ok(Uri {
                schema: self.schema.map(TryInto::try_into).transpose()?,
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
        pub schema: Option<schema::Tokenizer<'a>>,
        pub auth: Option<auth::Tokenizer<'a>>,
        pub host_with_port: host_with_port::Tokenizer<'a>,
        pub params: Vec<param::Tokenizer<'a>>,
        //TODO: why option here?
        pub headers: Option<Vec<&'a [u8]>>,
    }

    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
            use nom::{character::complete::space0, combinator::opt, multi::many0};

            let (rem, schema) = opt(schema::Tokenizer::tokenize)(part)?;
            let (rem, auth) = opt(auth::Tokenizer::tokenize)(rem)?;
            let (rem, host_with_port) = host_with_port::Tokenizer::tokenize(rem)?;
            let (rem, params) = many0(param::Tokenizer::tokenize)(rem)?;
            //TODO: remove these smart moves
            let (rem, _) = opt(space0)(rem)?;

            Ok((
                rem,
                Self {
                    schema,
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

            let (rem, schema) = opt(schema::Tokenizer::tokenize)(part)?;
            let (rem, auth) = opt(auth::Tokenizer::tokenize)(rem)?;
            let (rem, host_with_port) = host_with_port::Tokenizer::tokenize(rem)?;

            Ok((
                rem,
                Self {
                    schema,
                    auth,
                    host_with_port,
                    params: vec![],
                    headers: None,
                },
            ))
        }
    }
}
