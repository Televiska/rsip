use macros::UntypedHeader;

pub use super::www_authenticate::Tokenizer;

#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Authorization(String);

pub mod typed {
    use super::Tokenizer;
    use crate::{
        common::{
            auth::{self, Algorithm, AuthQop},
            Uri,
        },
        Error,
    };
    use macros::TypedHeader;
    use std::convert::{TryFrom, TryInto};

    #[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
    pub struct Authorization {
        pub scheme: auth::Scheme,
        pub username: String,
        pub realm: String,
        pub nonce: String,
        pub uri: Uri,
        pub response: String,
        pub algorithm: Option<Algorithm>,
        //TODO: support username* here in combination with userhash
        //TODO: this qop is not optional in rfc7616
        //also the cnonce and nc optional depends on qop
        //we should use an enum Qop with cnonce & nc fields instead
        //pub cnonce: Option<String>,
        pub opaque: Option<String>,
        //TODO: support multiple Qop
        pub qop: Option<AuthQop>,
        //TODO: this needs to be a u8
        //pub nc: Option<String>,
        //TODO: enable this in combination with username*
        //pub userhash: Option<bool>,
    }

    impl<'a> TryFrom<Tokenizer<'a>> for Authorization {
        type Error = crate::Error;

        fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
            Ok(Authorization {
                scheme: tokenizer.scheme.try_into()?,
                username: find_param(&tokenizer.params, "username")
                    .ok_or_else(|| Error::InvalidParam("missing username".into()))?
                    .into(),
                realm: find_param(&tokenizer.params, "realm")
                    .ok_or_else(|| Error::InvalidParam("missing realm".into()))?
                    .into(),
                nonce: find_param(&tokenizer.params, "nonce")
                    .ok_or_else(|| Error::InvalidParam("missing nonce".into()))?
                    .into(),
                uri: find_param(&tokenizer.params, "uri")
                    .ok_or_else(|| Error::InvalidParam("missing uri".into()))?
                    .try_into()?,
                response: find_param(&tokenizer.params, "response")
                    .ok_or_else(|| Error::InvalidParam("missing response".into()))?
                    .into(),
                algorithm: find_param(&tokenizer.params, "algorithm")
                    .map(TryInto::try_into)
                    .transpose()?,
                opaque: find_param(&tokenizer.params, "opaque").map(Into::into),
                qop: find_qop(&tokenizer.params)?,
            })
        }
    }

    impl std::fmt::Display for Authorization {
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

    fn find_qop<'a>(params: &[(&'a str, &'a str)]) -> Result<Option<AuthQop>, Error> {
        Ok(match find_param(params, "qop") {
            Some(qop) if qop.eq_ignore_ascii_case("auth") => Some(AuthQop::Auth {
                cnonce: find_param(params, "cnonce")
                    .ok_or_else(|| Error::InvalidParam("Found qop, but missing cnonce".into()))?
                    .into(),
                nc: find_param(params, "nc")
                    .ok_or_else(|| Error::InvalidParam("Found qop, but missing nc".into()))?
                    .parse::<u8>()?,
            }),
            Some(qop) if qop.eq_ignore_ascii_case("auth-int") => Some(AuthQop::AuthInt {
                cnonce: find_param(params, "cnonce")
                    .ok_or_else(|| Error::InvalidParam("Found qop, but missing cnonce".into()))?
                    .into(),
                nc: find_param(params, "nc")
                    .ok_or_else(|| Error::InvalidParam("Found qop, but missing nc".into()))?
                    .parse::<u8>()?,
            }),
            Some(qop) => return Err(Error::InvalidParam(format!("Found unknown qop: {}", qop))),
            None => None,
        })
    }
}
