use crate::headers::Header;
use macros::{Display, FromIntoInner, FromStr, HasValue, IntoHeader, Typed};
use std::convert::TryInto;

pub use tokenizer::Tokenizer;

#[derive(
    HasValue, Display, IntoHeader, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone, Typed,
)]
pub struct Contact(String);

pub mod tokenizer {
    use crate::common::uri;

    #[derive(Eq, PartialEq, Clone, Debug)]
    pub struct Tokenizer<'a> {
        pub display_name: Option<&'a str>,
        pub uri: uri::Tokenizer<'a>,
        pub params: Vec<uri::param::Tokenizer<'a>>,
    }

    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a str) -> Result<Self, crate::Error> {
            use nom::{
                bytes::complete::{tag, take_until},
                combinator::rest,
                error::VerboseError,
                multi::many0,
                sequence::tuple,
            };

            if part.contains('<') {
                let (_, (display_name, _, uri, _, params)) = tuple::<_, _, VerboseError<&str>, _>(
                    (take_until("<"), tag("<"), take_until(">"), tag(">"), rest),
                )(part)?;

                Ok(Self {
                    display_name: crate::utils::opt_trim(display_name),
                    uri: uri::Tokenizer::tokenize(uri.as_bytes())?.1,
                    params: many0(uri::param::Tokenizer::tokenize)(params.as_bytes())?.1,
                })
            } else {
                let (_, (uri, params)) = tuple((
                    uri::Tokenizer::tokenize_without_params,
                    many0(uri::param::Tokenizer::tokenize),
                ))(part.as_bytes())?;

                Ok(Self {
                    display_name: None,
                    uri,
                    params,
                })
            }
        }
    }
}

pub mod typed {
    use super::Tokenizer;
    use crate::common::{uri::Param, Uri};
    use macros::FromUntyped;
    use std::convert::{TryFrom, TryInto};

    #[derive(FromUntyped, Eq, PartialEq, Clone, Debug)]
    pub struct Contact {
        pub display_name: Option<String>,
        pub uri: Uri,
        pub params: Vec<Param>,
    }

    impl<'a> TryFrom<Tokenizer<'a>> for Contact {
        type Error = crate::Error;

        fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
            Ok(Contact {
                display_name: tokenizer.display_name.map(Into::into),
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
