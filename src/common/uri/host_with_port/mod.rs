mod host;
mod port;

#[doc(hidden)]
pub use tokenizer::Tokenizer;

pub use host::{Domain, Host};
pub use port::Port;

use crate::Error;
use std::convert::{TryFrom, TryInto};
use std::net::{IpAddr, SocketAddr};

/// Simple struct that holds the [Host] and [Port] of a SIP(S) uri, reprsented by [Uri](super::Uri).
/// Note that during parsing, if no port is set, it is returned as `None`. Usually when no port
/// is specified then port 5060 is assumed. But rsip is not acting smart here and delegates that
/// responsibility to you because you might want 5061 (TLS) as default etc.
///
/// Similarly on generation, if no port is specified, no port is set at all in the final string.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HostWithPort {
    pub host: Host,
    pub port: Option<Port>,
}

impl std::fmt::Display for HostWithPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.host, self.port.as_ref()) {
            (host, None) => write!(f, "{}", host),
            (host, Some(port)) => write!(f, "{}:{}", host, port),
        }
    }
}

impl TryFrom<String> for HostWithPort {
    type Error = Error;

    fn try_from(from: String) -> Result<Self, Self::Error> {
        from.as_str().try_into()
    }
}

impl TryFrom<&str> for HostWithPort {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        match from.rsplit_once(":") {
            None => Ok(Host::from(from).into()),
            Some((host, port)) => Ok((
                Host::from(String::from(host)),
                TryInto::<Port>::try_into(port)?,
            )
                .into()),
        }
    }
}

impl Default for HostWithPort {
    fn default() -> Self {
        Self {
            host: Default::default(),
            port: None,
        }
    }
}

impl From<Host> for HostWithPort {
    fn from(host: Host) -> Self {
        Self { host, port: None }
    }
}

impl From<IpAddr> for HostWithPort {
    fn from(from: IpAddr) -> Self {
        Self {
            host: from.into(),
            port: None,
        }
    }
}

impl From<SocketAddr> for HostWithPort {
    fn from(socket_addr: SocketAddr) -> Self {
        Self {
            host: Host::IpAddr(socket_addr.ip()),
            port: Some(socket_addr.port().into()),
        }
    }
}

impl TryInto<SocketAddr> for HostWithPort {
    type Error = Error;

    fn try_into(self) -> Result<SocketAddr, Error> {
        let ip_addr: IpAddr = self.host.try_into()?;

        Ok(SocketAddr::new(
            ip_addr,
            self.port.unwrap_or_else(|| 5060.into()).into(),
        ))
    }
}

impl From<Domain> for HostWithPort {
    fn from(domain: Domain) -> Self {
        Self {
            host: Host::Domain(domain),
            port: None,
        }
    }
}

impl<H, P> From<(H, P)> for HostWithPort
where
    H: Into<Host>,
    P: Into<Port>,
{
    fn from(from: (H, P)) -> Self {
        Self {
            host: from.0.into(),
            port: Some(from.1.into()),
        }
    }
}

impl<H, P> From<(H, Option<P>)> for HostWithPort
where
    H: Into<Host>,
    P: Into<Port>,
{
    fn from(from: (H, Option<P>)) -> Self {
        Self {
            host: from.0.into(),
            port: from.1.map(Into::into),
        }
    }
}

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a str, char>> for HostWithPort {
    type Error = Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a str, char>) -> Result<Self, Self::Error> {
        use std::str::FromStr;

        let host = match IpAddr::from_str(tokenizer.host) {
            Ok(ip_addr) => Host::IpAddr(ip_addr),
            Err(_) => Host::Domain(tokenizer.host.into()),
        };

        let port = match tokenizer.port {
            Some(port) => Some(port.parse::<u16>()?).map(Into::into),
            None => None,
        };

        Ok(Self { host, port })
    }
}

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a [u8], u8>> for HostWithPort {
    type Error = Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a [u8], u8>) -> Result<Self, Self::Error> {
        use std::str::from_utf8;

        Self::try_from(Tokenizer::from((
            from_utf8(tokenizer.host)?,
            tokenizer.port.map(from_utf8).transpose()?,
        )))
    }
}

#[doc(hidden)]
pub mod tokenizer {
    use crate::{AbstractInput, AbstractInputItem, GResult, GenericNomError, TokenizerError};
    use std::marker::PhantomData;

    #[derive(Debug, PartialEq, Eq, Clone, Default)]
    pub struct Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        pub host: T,
        pub port: Option<T>,
        phantom1: PhantomData<&'a T>,
        phantom2: PhantomData<I>,
    }

    impl<'a, T, I> From<(T, Option<T>)> for Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        fn from(from: (T, Option<T>)) -> Self {
            Self {
                host: from.0,
                port: from.1,
                phantom1: PhantomData,
                phantom2: PhantomData,
            }
        }
    }

    impl<'a, T, I> Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        pub fn tokenize(part: T) -> GResult<T, Self> {
            use nom::{
                bytes::complete::{tag, take_till1, take_until},
                combinator::rest,
                sequence::tuple,
            };

            let (rem, host_with_port) =
                take_till1(|c| c == Into::<I>::into(b';') || c == Into::<I>::into(b' '))(part)
                    .map_err(|_: GenericNomError<'a, T>| {
                        TokenizerError::from(("host with port", part)).into()
                    })?;

            let (host, port) = match tuple::<_, _, nom::error::VerboseError<T>, _>((
                take_until(":"),
                tag(":"),
                rest,
            ))(host_with_port)
            {
                Ok((_, (host, _, port))) => (host, Some(port)),
                Err(_) => {
                    //this is not going to ever fail actually, since rest never returns an
                    //error
                    let (_, host) = rest(host_with_port).map_err(|_: GenericNomError<'a, T>| {
                        TokenizerError::from(("host with port (no port)", host_with_port)).into()
                    })?;
                    (host, None)
                }
            };

            Ok((rem, (host, port).into()))
        }
    }
}
