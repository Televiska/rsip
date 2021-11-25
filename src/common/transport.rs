#[doc(hidden)]
pub use tokenizer::Tokenizer;

use crate::{Port, Scheme};

/// Simple enum that holds the transport type used (for instance in the `Via` header).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Transport {
    Udp,
    Tcp,
    Tls,
    TlsSctp,
    Sctp,
    Ws,
    Wss,
}

impl Transport {
    pub fn all() -> [Transport; 7] {
        use Transport::*;

        [Udp, Tcp, Tls, Sctp, TlsSctp, Ws, Wss]
    }

    pub fn protocols() -> [Transport; 4] {
        use Transport::*;

        [Udp, Tcp, Sctp, Ws]
    }

    pub fn secure_protocols() -> [Transport; 3] {
        use Transport::*;

        [Tcp, Sctp, Ws]
    }

    pub fn secure_transports() -> [Transport; 3] {
        use Transport::*;

        [Tls, TlsSctp, Wss]
    }

    pub fn default_secure_protocol() -> Self {
        Transport::Tcp
    }

    pub fn default_insecure_protocol() -> Self {
        Transport::Udp
    }

    pub fn default_secure_transport() -> Self {
        Transport::Tls
    }

    pub fn default_insecure_transport() -> Self {
        Transport::Udp
    }

    pub fn default_port(&self) -> Port {
        match self {
            Self::Udp => 5060.into(),
            Self::Tcp => 5060.into(),
            Self::Sctp => 5060.into(),
            Self::TlsSctp => 5061.into(),
            Self::Tls => 5061.into(),
            Self::Ws => 80.into(),
            Self::Wss => 443.into(),
        }
    }

    pub fn protocol(&self) -> Self {
        match self {
            Self::Tls => Self::Tcp,
            Self::TlsSctp => Self::Sctp,
            Self::Wss => Self::Ws,
            _ => *self,
        }
    }

    pub fn is_secure(&self) -> bool {
        Self::secure_transports().contains(self)
    }

    pub fn sip_scheme(&self) -> Scheme {
        match self.is_secure() {
            true => Scheme::Sips,
            false => Scheme::Sip,
        }
    }
}

impl Default for Transport {
    fn default() -> Self {
        Self::Udp
    }
}

impl std::fmt::Display for Transport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Udp => write!(f, "UDP"),
            Self::Tcp => write!(f, "TCP"),
            Self::Tls => write!(f, "TLS"),
            Self::Sctp => write!(f, "SCTP"),
            Self::TlsSctp => write!(f, "TLS-SCTP"),
            Self::Ws => write!(f, "WS"),
            Self::Wss => write!(f, "WSS"),
        }
    }
}

impl std::str::FromStr for Transport {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use std::convert::TryInto;

        Tokenizer {
            value: s.as_bytes(),
        }
        .try_into()
    }
}

#[doc(hidden)]
mod tokenizer {
    use super::Transport;
    use crate::{Error, IResult, NomError, TokenizerError};
    use std::convert::TryInto;

    //TODO: convert that to TryFrom, remove the need to parse utf8
    impl<'a> TryInto<Transport> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<Transport, Error> {
            use std::str::from_utf8;

            match from_utf8(self.value)? {
                part if part.eq_ignore_ascii_case("UDP") => Ok(Transport::Udp),
                part if part.eq_ignore_ascii_case("TCP") => Ok(Transport::Tcp),
                part if part.eq_ignore_ascii_case("TLS") => Ok(Transport::Tls),
                part if part.eq_ignore_ascii_case("SCTP") => Ok(Transport::Sctp),
                part if part.eq_ignore_ascii_case("TLS-SCTP") => Ok(Transport::TlsSctp),
                part if part.eq_ignore_ascii_case("WS") => Ok(Transport::Ws),
                part if part.eq_ignore_ascii_case("WSS") => Ok(Transport::Wss),
                part => Err(Error::ParseError(format!("unknown transport: {}", part))),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Tokenizer<'a> {
        pub value: &'a [u8],
    }

    impl<'a> From<&'a [u8]> for Tokenizer<'a> {
        fn from(value: &'a [u8]) -> Self {
            Self { value }
        }
    }

    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a [u8]) -> IResult<Self> {
            use nom::{branch::alt, bytes::complete::take_until1, combinator::rest};

            let (rem, transport) = alt((take_until1(" "), rest))(part)
                .map_err(|_: NomError<'a>| TokenizerError::from(("transport", part)).into())?;

            Ok((rem, transport.into()))
        }
    }
}

#[cfg(feature = "test-utils")]
impl testing_utils::Randomize for Transport {
    fn random() -> Self {
        testing_utils::sample(&Transport::protocols())
    }
}
