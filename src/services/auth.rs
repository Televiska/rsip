use crate::common::{
    auth::{Algorithm, Qop},
    uri::Uri,
    Method,
};

pub struct DigestGenerator<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub nonce: &'a str,
    pub cnonce: &'a str,
    pub nc: u32,
    pub uri: &'a Uri,
    pub realm: &'a str,
    pub method: Method,
    pub qop: Option<Qop>,
    pub algorithm: Algorithm,
}

impl<'a> DigestGenerator<'a> {
    pub fn compute(&self) -> String {
        let value = match self.qop {
            Some(ref qop) => format!(
                "{}:{}:{:08}:{}:{}:{}",
                self.ha1(),
                self.nonce,
                self.nc,
                self.cnonce,
                qop,
                self.ha2()
            ),
            None => format!("{}:{}:{}", self.ha1(), self.nonce, self.ha2()),
        };

        self.hash_value(value)
    }

    fn ha1(&self) -> String {
        let value = format!("{}:{}:{}", self.username, self.realm, self.password);

        self.hash_value(value)
    }

    fn ha2(&self) -> String {
        let value = match self.qop {
            None | Some(Qop::Auth) => format!("{}:{}", self.method, self.uri),
            _ => format!(
                "{}:{}:d41d8cd98f00b204e9800998ecf8427e",
                self.method, self.uri
            ),
        };

        self.hash_value(value)
    }

    fn hash_value(&self, value: String) -> String {
        use md5::{Digest, Md5};
        use sha2::{Sha256, Sha512};

        match self.algorithm {
            Algorithm::Md5 | Algorithm::Md5Sess => {
                let mut hasher = Md5::new();
                hasher.update(value);
                format!("{:x}", hasher.finalize())
            }
            Algorithm::Sha256 | Algorithm::Sha256Sess => {
                let mut hasher = Sha256::new();
                hasher.update(value);
                format!("{:x}", hasher.finalize())
            }
            Algorithm::Sha512 | Algorithm::Sha512Sess => {
                let mut hasher = Sha512::new();
                hasher.update(value);
                format!("{:x}", hasher.finalize())
            }
        }
    }
}
