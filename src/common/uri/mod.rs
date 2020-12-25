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
    pub schema: Option<Schema>,
    pub host_with_port: HostWithPort,
    pub auth: Option<Auth>,
    pub params: Vec<Param>,
}

impl Uri {
    pub fn username(&self) -> Option<String> {
        self.auth.as_ref().map(|auth| auth.username.clone())
    }

    pub fn domain(&self) -> String {
        self.host_with_port.clone().domain()
    }

    pub fn port(&self) -> u16 {
        self.host_with_port.clone().port()
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
            auth: Default::default(),
            params: Default::default(),
        }
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<libsip::uri::Uri>::into(self.clone()))
    }
}

impl Into<libsip::uri::Uri> for Uri {
    fn into(self) -> libsip::uri::Uri {
        libsip::uri::Uri {
            schema: self.schema.map(Into::into),
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
            schema: from.schema.map(Into::into),
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
