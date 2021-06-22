#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Qop {
    Auth,
    AuthInt,
}

impl Default for Qop {
    fn default() -> Self {
        Self::Auth
    }
}

impl std::fmt::Display for Qop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auth => write!(f, "auth"),
            Self::AuthInt => write!(f, "auth-int"),
        }
    }
}

impl std::str::FromStr for Qop {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use std::convert::TryInto;

        s.try_into()
    }
}

impl std::convert::TryFrom<&str> for Qop {
    type Error = crate::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            s if s.eq_ignore_ascii_case("auth") => Ok(Self::Auth),
            s if s.eq_ignore_ascii_case("auth-int") => Ok(Self::AuthInt),
            s => Err(crate::Error::ParseError(format!("invalid Qop `{}`", s))),
        }
    }
}
