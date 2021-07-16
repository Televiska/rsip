pub use tokenizer::Tokenizer;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Transport {
    Udp,
    Tcp,
    Tls,
    Sctp,
    //Other(String),
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
            //Self::Other(inner) => write!(f, "{}", inner),
        }
    }
}

mod tokenizer {
    use super::Transport;
    use crate::{Error, NomError};
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
        pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
            use nom::{branch::alt, bytes::complete::take_until, combinator::rest};

            let (rem, transport) = alt((take_until(" "), rest))(part)?;

            Ok((rem, transport.into()))
        }
    }
}
