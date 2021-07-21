/// The `Algorithm`, as part of the SIP Authorization framework, found in headers like
/// [Authorization](super::super::typed::Authorization) and
/// [WwwAuthenticate](super::super::typed::WwwAuthenticate)
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Qop {
    Auth,
    AuthInt,
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AuthQop {
    Auth { cnonce: String, nc: u8 },
    AuthInt { cnonce: String, nc: u8 },
}

impl std::fmt::Display for AuthQop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auth { cnonce, nc } => {
                write!(f, "qop=\"auth\", nc={:08}, cnonce=\"{}\"", nc, cnonce)
            }
            Self::AuthInt { cnonce, nc } => {
                write!(f, "qop=\"auth-int\", nc={:08}, cnonce=\"{}\"", nc, cnonce)
            }
        }
    }
}
