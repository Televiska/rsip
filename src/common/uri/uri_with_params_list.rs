use crate::{common::uri::UriWithParams, Error};

#[doc(hidden)]
pub use tokenizer::Tokenizer;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct UriWithParamsList(pub Vec<UriWithParams>);

impl std::fmt::Display for UriWithParamsList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a str, char>> for UriWithParamsList {
    type Error = Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a str, char>) -> Result<Self, Self::Error> {
        Ok(Self(
            tokenizer
                .values
                .into_iter()
                .map(UriWithParams::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a [u8], u8>> for UriWithParamsList {
    type Error = Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a [u8], u8>) -> Result<Self, Self::Error> {
        Ok(Self(
            tokenizer
                .values
                .into_iter()
                .map(UriWithParams::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[doc(hidden)]
pub mod tokenizer {
    use crate::{
        common::uri::uri_with_params, AbstractInput, AbstractInputItem, GResult, TokenizerError,
    };
    use std::marker::PhantomData;

    #[derive(Debug, PartialEq, Eq, Clone, Default)]
    pub struct Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        pub values: Vec<uri_with_params::Tokenizer<'a, T, I>>,
        pub phantom1: PhantomData<&'a T>,
        pub phantom2: PhantomData<I>,
    }

    impl<'a, T, I> Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
        TokenizerError: nom::error::ParseError<T>,
    {
        //uses comma as a stopbreak
        //can parse both enclosed and plain
        //expects to eat all input
        pub fn tokenize(part: T) -> GResult<T, Self> {
            use crate::parser_utils::is_empty_or_fail_with;
            use nom::multi::many1;

            let (rem, uri_with_params) = many1(uri_with_params::Tokenizer::tokenize)(part)
                .map_err(|_| TokenizerError::from(("uri with params", part)).into())?;
            is_empty_or_fail_with(
                rem,
                (
                    "uri with params list tokenization left trailing input",
                    part,
                ),
            )?;

            Ok((
                rem,
                Self {
                    values: uri_with_params,
                    phantom1: Default::default(),
                    phantom2: Default::default(),
                },
            ))
        }
    }
}
