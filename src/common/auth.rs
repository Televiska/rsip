use crate::Error;
use std::convert::{TryFrom, TryInto};

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

impl<'a> From<Algorithm> for &'a str {
    fn from(from: Algorithm) -> Self {
        from.algo.into()
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

impl<'a> From<AlgorithmType> for &'a str {
    fn from(from: AlgorithmType) -> Self {
        match from {
            AlgorithmType::Md5 => "MD5",
            AlgorithmType::Sha256 => "SHA-256",
            AlgorithmType::Sha512 => "SHA-512-256",
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
            s => Err(Error::InvalidParam(format!(
                "invalid AlgorithmType: `{}`",
                s
            ))),
        }
    }
}

impl From<Qop> for String {
    fn from(from: Qop) -> Self {
        match from {
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
            s => Err(Error::InvalidParam(format!("invalid Qop `{}`", s))),
        }
    }
}
