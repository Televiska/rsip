use macros::UntypedHeader;

pub use super::www_authenticate::Tokenizer;

#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Authorization(String);

pub mod typed {
    use super::Tokenizer;
    use crate::{common::auth, Error};
    use macros::TypedHeader;
    use std::convert::{TryFrom, TryInto};

    #[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
    pub struct Authorization {
        pub scheme: auth::Scheme,
        pub username: String,
        pub realm: String,
        pub nonce: String,
        pub uri: String,
        pub response: String,
        pub algorithm: Option<String>,
        //TODO: support username* here in combination with userhash
        //TODO: this qop is not optional in rfc7616
        //also the cnonce and nc optional depends on qop
        //we should use an enum Qop with cnonce & nc fields instead
        pub cnonce: Option<String>,
        pub opaque: Option<String>,
        pub qop: Option<String>,
        pub nc: Option<String>,
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
                    .into(),
                response: find_param(&tokenizer.params, "response")
                    .ok_or_else(|| Error::InvalidParam("missing response".into()))?
                    .into(),
                algorithm: find_param(&tokenizer.params, "algorithm").map(Into::into),
                cnonce: find_param(&tokenizer.params, "cnonce").map(Into::into),
                opaque: find_param(&tokenizer.params, "opaque").map(Into::into),
                qop: find_param(&tokenizer.params, "qop").map(Into::into),
                nc: find_param(&tokenizer.params, "nc").map(Into::into),
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
}
