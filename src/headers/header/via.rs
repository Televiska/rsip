use crate::headers::Header;
use macros::{Display, FromIntoInner, FromStr, HasValue, IntoHeader, Typed};
use std::convert::TryInto;

pub use tokenizer::Tokenizer;

#[derive(
    HasValue, Display, IntoHeader, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone, Typed,
)]
pub struct Via(String);

pub mod tokenizer {
    use crate::common::{
        transport,
        uri::{self, param},
        version,
    };

    #[derive(Eq, PartialEq, Debug)]
    pub struct Tokenizer<'a> {
        pub version: version::Tokenizer<'a>,
        pub transport: transport::Tokenizer<'a>,
        pub uri: uri::Tokenizer<'a>,
        pub params: Vec<param::Tokenizer<'a>>,
    }

    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a str) -> Result<Self, crate::Error> {
            use nom::{
                bytes::complete::tag, character::complete::space1, multi::many0, sequence::tuple,
            };

            let (_, (version, _, transport, _, uri, params)) = tuple((
                version::Tokenizer::tokenize,
                tag("/"),
                transport::Tokenizer::tokenize,
                space1,
                uri::Tokenizer::tokenize_without_params,
                many0(param::Tokenizer::tokenize),
            ))(part.as_bytes())?;

            Ok(Self {
                version,
                transport,
                uri,
                params,
            })
        }
    }
}

pub mod typed {
    use super::Tokenizer;
    use crate::common::{
        uri::{self, Uri},
        Transport, Version,
    };
    use macros::FromUntyped;
    use std::convert::{TryFrom, TryInto};

    #[derive(FromUntyped, Eq, PartialEq, Clone, Debug)]
    pub struct Via {
        pub version: Version,
        pub transport: Transport,
        pub uri: Uri,
        pub params: Vec<uri::Param>,
    }

    impl<'a> TryFrom<Tokenizer<'a>> for Via {
        type Error = crate::Error;

        fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
            Ok(Via {
                version: tokenizer.version.try_into()?,
                transport: tokenizer.transport.try_into()?,
                uri: tokenizer.uri.try_into()?,
                params: tokenizer
                    .params
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()?,
            })
        }
    }
}
