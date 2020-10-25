//TODO: this needs some love

use crate::common::{
    auth::{Algorithm, Qop, Error},
};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Authorization {
    pub realm: String,
    pub nonce: String,
    pub opaque: Option<String>,
    //pub userhash: bool,
    pub algorithm: Algorithm,
    pub response: Option<String>,
    pub username: String,
    pub uri: String,
    pub qop: Option<Qop>,
    pub cnonce: Option<String>,
    pub nc: Option<u32>,
}

impl Authorization {
    pub fn with_digest_for(&mut self, password: String) {
        let cnonce = self
            .cnonce
            .clone()
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        let nc = self.nc.unwrap_or(1);
        let ha1 = md5::compute(&format!("{}:{}:{}", self.username, self.realm, password));
        let ha2 = md5::compute(format!("REGISTER:{}", self.uri));
        let digest = format!(
            "{:x}:{}:{:08}:{}:auth:{:x}",
            ha1, self.nonce, nc, cnonce, ha2
        );

        self.cnonce = Some(cnonce);
        self.nc = Some(nc);

        self.response = Some(format!("{:x}", md5::compute(digest)));
    }

    pub fn verify_for(&self, password: String) -> Result<bool, Error> {
        let response = match &self.response {
            Some(response) => response,
            None => return Err("missing part: response".into()),
        };
        let cnonce = match &self.cnonce {
            Some(cnonce) => cnonce,
            None => return Err("missing part: cnonce".into()),
        };
        let nc = match self.nc {
            Some(nc) => nc,
            None => return Err("missing part: nc".into()),
        };

        let ha1 = md5::compute(&format!("{}:{}:{}", self.username, self.realm, password));
        let ha2 = md5::compute(format!("REGISTER:{}", self.uri));
        let digest = format!(
            "{:x}:{}:{:08}:{}:auth:{:x}",
            ha1, self.nonce, nc, cnonce, ha2
        );
        let digest = format!("{:x}", md5::compute(digest));

        Ok(digest == *response)
    }
}

impl Into<libsip::headers::AuthHeader> for Authorization {
    fn into(self) -> libsip::headers::AuthHeader {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("realm".into(), self.realm);
        map.insert("nonce".into(), self.nonce);
        if let Some(opaque) = self.opaque {
            map.insert("opaque".into(), opaque);
        }
        map.insert("algorithm".into(), self.algorithm.into());
        if let Some(response) = self.response {
            map.insert("response".into(), response);
        }
        map.insert("username".into(), self.username);
        map.insert("uri".into(), self.uri);
        if let Some(qop) = self.qop {
            map.insert("qop".into(), qop.into());
        }
        if let Some(cnonce) = self.cnonce {
            map.insert("cnonce".into(), cnonce);
        }
        if let Some(nc) = self.nc {
            map.insert("nc".into(), nc.to_string());
        }

        libsip::headers::AuthHeader(libsip::headers::AuthSchema::Digest, map)
    }
}

impl TryFrom<libsip::headers::AuthHeader> for Authorization {
    type Error = Error;

    fn try_from(header: libsip::headers::AuthHeader) -> Result<Self, Self::Error> {
        //let schema = header.0;
        let map = header.1;

        Ok(Self {
            realm: get_from(&map, "realm")?,
            nonce: get_from(&map, "nonce")?,
            opaque: None,
            algorithm: get_from(&map, "algorithm")
                .unwrap_or_else(|_| "md5".into())
                .try_into()?,
            response: Some(get_from(&map, "response")?),
            username: get_from(&map, "username")?,
            uri: get_from(&map, "uri")?,
            qop: get_from(&map, "qop")
                .ok()
                .map(|s| s.try_into())
                .transpose()?,
            cnonce: get_from(&map, "cnonce").ok(),
            nc: get_from(&map, "nc")
                .ok()
                .map(|s| s.parse::<u32>().map_err(|e| e.to_string()))
                .transpose()?,
        })
    }
}

fn get_from(map: &HashMap<String, String>, part: &str) -> Result<String, Error> {
    Ok(map
        .get(part)
        .ok_or_else(|| format!("missing part: {}", Into::<String>::into(part)))?
        .to_string())
}


impl Into<libsip::headers::Header> for Authorization {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::Authorization(self.into())
    }
}

