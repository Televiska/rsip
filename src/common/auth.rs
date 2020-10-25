//TODO: these need some love

use std::convert::{TryFrom, TryInto};

pub type Error = String;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Qop {
    Auth,
    AuthInt,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AlgorithmType {
    Md5,
    Sha256,
    Sha512,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Algorithm {
    pub algo: AlgorithmType,
    //pub sess: bool,
}

impl Default for Algorithm {
    fn default() -> Self {
        Self {
            algo: AlgorithmType::Md5,
        }
    }
}

impl Into<String> for Algorithm {
    fn into(self) -> String {
        self.algo.into()
    }
}

impl TryFrom<String> for Algorithm {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self {
            algo: s.try_into()?,
        })
    }
}

impl Into<String> for AlgorithmType {
    fn into(self) -> String {
        match self {
            Self::Md5 => "md5".into(),
            Self::Sha256 => "SHA-256".into(),
            Self::Sha512 => "SHA-512-256".into(),
        }
    }
}

impl TryFrom<String> for AlgorithmType {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s {
            s if s.eq_ignore_ascii_case("md5") => Ok(AlgorithmType::Md5),
            s if s.eq_ignore_ascii_case("sha-256") => Ok(AlgorithmType::Sha256),
            s if s.eq_ignore_ascii_case("sha-512-256") => Ok(AlgorithmType::Sha512),
            s => Err(format!("invalid AlgorithmType `{}`", s)),
        }
    }
}

impl Into<String> for Qop {
    fn into(self) -> String {
        match self {
            Qop::Auth => "auth".into(),
            Qop::AuthInt => "auth-int".into(),
        }
    }
}

impl TryFrom<String> for Qop {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s {
            s if s.eq_ignore_ascii_case("auth") => Ok(Qop::Auth),
            s if s.eq_ignore_ascii_case("auth-int") => Ok(Qop::AuthInt),
            s => Err(format!("invalid Qop `{}`", s)),
        }
    }
}
