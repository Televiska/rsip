#[doc(hidden)]
pub use tokenizer::Tokenizer;

use crate::{Error, Port, Scheme};

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
        use std::convert::TryFrom;

        Self::try_from(Tokenizer::from(s))
    }
}

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a str, char>> for Transport {
    type Error = Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a str, char>) -> Result<Self, Self::Error> {
        match tokenizer.value {
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

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a [u8], u8>> for Transport {
    type Error = Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a [u8], u8>) -> Result<Self, Self::Error> {
        use std::str::from_utf8;

        let value = from_utf8(tokenizer.value)?;

        Self::try_from(Tokenizer::from(value))
    }
}

#[doc(hidden)]
mod tokenizer {
    use crate::{AbstractInput, AbstractInputItem, GResult, GenericNomError, TokenizerError};
    use std::marker::PhantomData;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        pub value: T,
        phantom1: PhantomData<&'a T>,
        phantom2: PhantomData<I>,
    }

    impl<'a, T, I> From<T> for Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        fn from(value: T) -> Self {
            Self {
                value,
                phantom1: Default::default(),
                phantom2: Default::default(),
            }
        }
    }

    impl<'a, T, I> Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        pub fn tokenize(part: T) -> GResult<T, Self> {
            use nom::{branch::alt, bytes::complete::take_until1, combinator::rest};

            let (rem, transport) =
                alt((take_until1(" "), rest))(part).map_err(|_: GenericNomError<'a, T>| {
                    TokenizerError::from(("transport", part)).into()
                })?;

            Ok((rem, Tokenizer::from(transport)))
        }
    }
}

#[cfg(feature = "test-utils")]
impl testing_utils::Randomize for Transport {
    fn random() -> Self {
        testing_utils::sample(&Transport::protocols())
    }
}
