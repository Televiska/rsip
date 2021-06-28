use macros::UntypedHeader;

pub use tokenizer::Tokenizer;

#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
#[header(display_name = "WWW-Authenticate")]
pub struct WwwAuthenticate(String);

pub mod tokenizer {
    use crate::{common::auth, headers::header::Tokenize};

    #[derive(Eq, PartialEq, Clone, Debug)]
    pub struct Tokenizer<'a> {
        pub scheme: auth::scheme::Tokenizer<'a, &'a str>,
        pub params: Vec<(&'a str, &'a str)>,
    }

    impl<'a> Tokenize<'a> for Tokenizer<'a> {
        fn tokenize(part: &'a str) -> Result<Self, crate::Error> {
            use nom::{
                branch::alt,
                bytes::complete::{tag, take_until},
                character::complete::space0,
                combinator::{map, opt, rest},
                error::VerboseError,
                multi::many1,
                sequence::{delimited, tuple},
            };

            let params = map::<_, _, _, VerboseError<&str>, _, _>(
                tuple((
                    space0,
                    take_until("="),
                    tag("="),
                    alt((
                        delimited(tag("\""), take_until("\""), tag("\"")),
                        take_until(","),
                        take_until(" "),
                        rest,
                    )),
                    opt(tag(",")),
                )),
                |tuple| (tuple.1, tuple.3),
            );
            let (_, (_, scheme, params)) =
                tuple((space0, auth::scheme::Tokenizer::tokenize, many1(params)))(part)?;

            let params = params
                .into_iter()
                .map(|(key, value)| {
                    if value.starts_with('"') && value.ends_with('"') {
                        (key, &value[1..(value.len() - 1)])
                    } else {
                        (key, value)
                    }
                })
                .collect::<Vec<_>>();

            Ok(Self { scheme, params })
        }
    }
}

pub mod typed {
    use super::Tokenizer;
    use crate::common::auth::{Algorithm, Qop};
    use crate::{common::auth, Error};
    use macros::TypedHeader;
    use std::convert::{TryFrom, TryInto};

    #[derive(TypedHeader, Eq, PartialEq, Clone, Debug, Default)]
    pub struct WwwAuthenticate {
        pub scheme: auth::Scheme,
        pub realm: String,
        pub domain: Option<String>,
        pub nonce: String,
        pub opaque: Option<String>,
        pub stale: Option<String>,
        pub algorithm: Option<Algorithm>,
        //TODO: support multiple Qop
        pub qop: Option<Qop>,
        pub charset: Option<String>,
        //pub userhash: Option<bool>,
    }

    impl<'a> TryFrom<Tokenizer<'a>> for WwwAuthenticate {
        type Error = crate::Error;

        fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
            Ok(WwwAuthenticate {
                scheme: tokenizer.scheme.try_into()?,
                realm: find_param(&tokenizer.params, "realm")
                    .ok_or_else(|| Error::InvalidParam("missing realm".into()))?
                    .into(),
                domain: find_param(&tokenizer.params, "domain").map(Into::into),
                nonce: find_param(&tokenizer.params, "nonce")
                    .ok_or_else(|| Error::InvalidParam("missing nonce".into()))?
                    .into(),
                opaque: find_param(&tokenizer.params, "opaque").map(Into::into),
                stale: find_param(&tokenizer.params, "stale").map(Into::into),
                algorithm: find_param(&tokenizer.params, "algorithm")
                    .map(TryInto::try_into)
                    .transpose()?,
                qop: find_param(&tokenizer.params, "qop")
                    .map(TryInto::try_into)
                    .transpose()?,
                charset: find_param(&tokenizer.params, "charset").map(Into::into),
            })
        }
    }

    impl std::fmt::Display for WwwAuthenticate {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "unimplemented",)
        }
    }

    fn find_param<'a>(params: &[(&'a str, &'a str)], name: &str) -> Option<&'a str> {
        params.iter().find_map(|(key, value)| {
            if key.eq_ignore_ascii_case(name) {
                Some(*value)
            } else {
                None
            }
        })
    }
}
