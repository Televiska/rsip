//TODO: this needs some love

use crate::common::auth::{Algorithm, Qop};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WwwAuthenticate {
    //that should be vec
    pub domain: Option<String>,
    pub realm: String,
    pub nonce: String,
    pub opaque: Option<String>,
    pub stale: bool,
    pub algorithm: Algorithm,
    //latest rfc requires qop
    pub qop: Option<Qop>,
    //pub userhash: bool,
    //pub charset: Charset,
}

impl WwwAuthenticate {
    pub fn new(realm: String, nonce: String) -> Self {
        Self {
            domain: Some(realm.clone()),
            realm,
            nonce,
            opaque: None,
            stale: false,
            algorithm: Algorithm::default(),
            qop: Some(Qop::Auth),
        }
    }
}

impl Into<libsip::headers::AuthHeader> for WwwAuthenticate {
    fn into(self) -> libsip::headers::AuthHeader {
        let mut map: HashMap<String, String> = HashMap::new();
        if let Some(domain) = self.domain {
            map.insert("domain".into(), domain);
        }
        map.insert("realm".into(), self.realm);
        map.insert("nonce".into(), self.nonce);
        if let Some(opaque) = self.opaque {
            map.insert("opaque".into(), opaque);
        }
        match self.stale {
            true => map.insert("stale".into(), "TRUE".into()),
            false => map.insert("stale".into(), "FALSE".into()),
        };

        map.insert("algorithm".into(), self.algorithm.into());
        if let Some(qop) = self.qop {
            map.insert("qop".into(), qop.into());
        }

        libsip::headers::AuthHeader(libsip::headers::AuthSchema::Digest, map)
    }
}

impl Into<libsip::headers::Header> for WwwAuthenticate {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::WwwAuthenticate(self.into())
    }
}

