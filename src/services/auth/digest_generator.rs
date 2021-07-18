use crate::{
    common::{uri::Uri, Method},
    headers::{
        self,
        auth::{Algorithm, AuthQop},
    },
};

/// Simple helpful struct to generate & verify the `Digest` authentication strings.
/// It can also be created from an [Authorization](crate::typed::Authorization),
/// an `&str` for the password and a [Method](crate::Method) using the [from](From::from()) method.
///
/// Supports SIP versions of [RFC7616](https://datatracker.ietf.org/doc/html/rfc7616)
/// and [RFC2617](https://datatracker.ietf.org/doc/html/rfc2617).
#[derive(Debug, Clone)]
pub struct DigestGenerator<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub nonce: &'a str,
    pub uri: &'a Uri,
    pub realm: &'a str,
    pub method: &'a Method,
    pub qop: Option<&'a AuthQop>,
    pub algorithm: Algorithm,
}

impl<'a> DigestGenerator<'a> {
    //TODO: log if scheme is not digest
    pub fn from(
        auth: &'a headers::typed::Authorization,
        password: &'a str,
        method: &'a Method,
    ) -> Self {
        Self {
            username: &auth.username,
            password,
            nonce: &auth.nonce,
            uri: &auth.uri,
            realm: &auth.realm,
            method: &method,
            qop: auth.qop.as_ref(),
            algorithm: auth.algorithm.unwrap_or(Algorithm::Md5),
        }
    }

    pub fn verify(&self, response: &'a str) -> bool {
        self.compute() == response
    }

    pub fn compute(&self) -> String {
        let value = match self.qop {
            Some(AuthQop::Auth { cnonce, nc }) => format!(
                "{}:{}:{:08}:{}:{}:{}",
                self.ha1(),
                self.nonce,
                nc,
                cnonce,
                "auth",
                self.ha2()
            ),
            Some(AuthQop::AuthInt { cnonce, nc }) => format!(
                "{}:{}:{:08}:{}:{}:{}",
                self.ha1(),
                self.nonce,
                nc,
                cnonce,
                "auth-int",
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
            None | Some(AuthQop::Auth { .. }) => format!("{}:{}", self.method, self.uri),
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
