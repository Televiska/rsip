use macros::UntypedHeader;

pub use tokenizer::Tokenizer;

#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Via(String);

pub mod tokenizer {
    use crate::{
        common::{
            transport,
            uri::{self, param},
            version,
        },
        headers::header::Tokenize,
    };

    #[derive(Eq, PartialEq, Debug)]
    pub struct Tokenizer<'a> {
        pub version: version::Tokenizer<'a>,
        pub transport: transport::Tokenizer<'a>,
        pub uri: uri::Tokenizer<'a>,
        pub params: Vec<param::Tokenizer<'a>>,
    }

    impl<'a> Tokenize<'a> for Tokenizer<'a> {
        fn tokenize(part: &'a str) -> Result<Self, crate::Error> {
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
        uri::{self, param::Branch, Param, Uri},
        Transport, Version,
    };
    use macros::{TypedHeader, UriAndParamsHelpers};
    use std::convert::{TryFrom, TryInto};

    #[derive(TypedHeader, UriAndParamsHelpers, Eq, PartialEq, Clone, Debug)]
    pub struct Via {
        pub version: Version,
        pub transport: Transport,
        pub uri: Uri,
        pub params: Vec<uri::Param>,
    }

    impl Via {
        pub fn branch(&self) -> Option<&Branch> {
            self.params.iter().find_map(|param| match param {
                Param::Branch(branch) => Some(branch),
                _ => None,
            })
        }
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

    impl std::fmt::Display for Via {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}/{} {}{}",
                self.version,
                self.transport,
                self.uri,
                self.params
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            )
        }
    }

    impl std::convert::From<crate::common::Uri> for Via {
        fn from(uri: crate::common::Uri) -> Self {
            Self {
                version: Default::default(),
                transport: Default::default(),
                uri,
                params: vec![Param::Branch(Default::default())],
            }
        }
    }
}
