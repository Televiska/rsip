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

/*
//TODO: does this make sense? Param could have multiple variants that Domain is used
impl Into<Param> for Domain {
    fn into(self) -> Param {
        Param::Received(self)
    }
}
//TODO: same here
impl Into<libsip::uri::UriParam> for Domain {
    fn into(self) -> libsip::uri::UriParam {
        libsip::uri::UriParam::Received(self.into())
    }
}
*/

//TODO: improve here (probably not needed at all)
/*
impl TryFrom<libsip::uri::Domain> for Domain {
    type Error = String;

    fn try_from(from: libsip::uri::Domain) -> Result<Self, Self::Error> {
        match from {
            libsip::uri::Domain::Domain(host, port) => Ok(Domain { host, port }),
            libsip::uri::Domain::Ipv4(ip, port) => Err(format!(
                "Can't convert libsip::uri::Domain to Domain, given ip address {} {:?}",
                ip, port
            )
            .into()),
        }
    }
}*/
