#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Algorithm {
    Md5,
    Md5Sess,
    Sha256,
    Sha256Sess,
    Sha512,
    Sha512Sess,
}

impl Default for Algorithm {
    fn default() -> Self {
        Self::Sha256
    }
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Md5 => write!(f, "md5"),
            Self::Md5Sess => write!(f, "md5-sess"),
            Self::Sha256 => write!(f, "sha256"),
            Self::Sha256Sess => write!(f, "sha256-sess"),
            Self::Sha512 => write!(f, "sha512"),
            Self::Sha512Sess => write!(f, "sha512-sess"),
        }
    }
}

impl std::str::FromStr for Algorithm {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use std::convert::TryInto;

        s.try_into()
    }
}

impl std::convert::TryFrom<&str> for Algorithm {
    type Error = crate::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            s if s.eq_ignore_ascii_case("md5") => Ok(Self::Md5),
            s if s.eq_ignore_ascii_case("md5-sess") => Ok(Self::Md5Sess),
            s if s.eq_ignore_ascii_case("sha256") => Ok(Self::Sha256),
            s if s.eq_ignore_ascii_case("sha256-sess") => Ok(Self::Sha256Sess),
            s if s.eq_ignore_ascii_case("sha512") => Ok(Self::Sha512),
            s if s.eq_ignore_ascii_case("sha512-sess") => Ok(Self::Sha512Sess),
            s => Err(crate::Error::ParseError(format!(
                "invalid Algorithm `{}`",
                s
            ))),
        }
    }
}
