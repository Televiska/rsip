//use crate::common::uri::Param;
//use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Domain {
    pub host: String,
    pub port: Option<u16>,
}

//TODO: host default here should be compiled-configured
impl Default for Domain {
    fn default() -> Self {
        Self {
            host: "example.com".into(),
            port: Some(5060),
        }
    }
}

impl From<(String, Option<u16>)> for Domain {
    fn from(tuple: (String, Option<u16>)) -> Self {
        Self {
            host: tuple.0,
            port: tuple.1,
        }
    }
}

impl Into<libsip::uri::Domain> for Domain {
    fn into(self) -> libsip::uri::Domain {
        libsip::uri::Domain::Domain(self.host, self.port)
    }
}
