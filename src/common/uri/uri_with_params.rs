use crate::{
    common::{uri::Param, Uri},
    Error,
};
use std::convert::TryInto;

#[doc(hidden)]
pub use tokenizer::Tokenizer;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct UriWithParams {
    pub uri: Uri,
    pub params: Vec<Param>,
}

impl UriWithParams {
    pub fn is_sips(&self) -> Result<bool, Error> {
        self.uri.is_sips()
    }
}

impl std::fmt::Display for UriWithParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}>{}",
            self.uri,
            self.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

impl<U, P> From<(U, P)> for UriWithParams
where
    U: Into<Uri>,
    P: Into<Param>,
{
    fn from(tuple: (U, P)) -> Self {
        Self {
            uri: tuple.0.into(),
            params: vec![tuple.1.into()],
        }
    }
}

impl<U, P> From<(U, Vec<P>)> for UriWithParams
where
    U: Into<Uri>,
    P: Into<Param>,
{
    fn from(tuple: (U, Vec<P>)) -> Self {
        Self {
            uri: tuple.0.into(),
            params: tuple.1.into_iter().map(Into::into).collect(),
        }
    }
}

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a str, char>> for UriWithParams {
    type Error = Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a str, char>) -> Result<Self, Self::Error> {
        Ok(Self {
            uri: tokenizer.uri.try_into()?,
            params: tokenizer
                .params
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a [u8], u8>> for UriWithParams {
    type Error = Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a [u8], u8>) -> Result<Self, Self::Error> {
        Ok(Self {
            uri: tokenizer.uri.try_into()?,
            params: tokenizer
                .params
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[doc(hidden)]
pub mod tokenizer {
    use crate::{
        common::{param, uri},
        AbstractInput, AbstractInputItem, GResult, TokenizerError,
    };
    use std::marker::PhantomData;

    #[derive(Debug, PartialEq, Eq, Clone, Default)]
    pub struct Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        pub uri: uri::Tokenizer<'a, T, I>,
        pub params: Vec<param::Tokenizer<'a, T, I>>,
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
            use nom::branch::alt;

            alt((Self::tokenize_enclosed, Self::tokenize_plain))(part)
        }

        pub fn tokenize_plain(part: T) -> GResult<T, Self> {
            use crate::parser_utils::is_empty_or_fail_with;
            use nom::{
                branch::alt,
                bytes::complete::{tag, take_until},
                combinator::{map, rest},
                multi::many0,
                sequence::tuple,
            };

            let stopbreak = alt((
                map(tuple((take_until(","), tag(","))), |(value, _)| value),
                rest,
            ));

            let (rem, (uri, params)) =
                tuple((uri::Tokenizer::tokenize_without_params, stopbreak))(part)?;

            let (params_rem, params) = many0(uri::param::Tokenizer::tokenize)(params)
                .map_err(|_| TokenizerError::from(("params", part)).into())?;
            is_empty_or_fail_with(params_rem, ("params tokenizing left trailing input", part))?;

            Ok((
                rem,
                Self {
                    uri,
                    params,
                    phantom1: Default::default(),
                    phantom2: Default::default(),
                },
            ))
        }

        pub fn tokenize_enclosed(part: T) -> GResult<T, Self> {
            use crate::parser_utils::is_empty_or_fail_with;
            use nom::{
                branch::alt,
                bytes::complete::{tag, take_until},
                character::complete::space0,
                combinator::{map, rest},
                error::VerboseError,
                multi::many0,
                sequence::tuple,
            };

            let stopbreak = alt((
                map(tuple((take_until(","), tag(","))), |(value, _)| value),
                rest,
            ));

            let (rem, (_, _, uri, _, params)) = tuple::<_, _, VerboseError<T>, _>((
                space0,
                tag("<"),
                take_until(">"),
                tag(">"),
                stopbreak,
            ))(part)
            .map_err(|_| TokenizerError::from(("header parts", part)).into())?;

            let (params_rem, params) = many0(uri::param::Tokenizer::tokenize)(params)
                .map_err(|_| TokenizerError::from(("params", part)).into())?;
            is_empty_or_fail_with(params_rem, ("params tokenizing left trailing input", part))?;

            Ok((
                rem,
                Self {
                    uri: uri::Tokenizer::tokenize(uri)
                        .map_err(|_| TokenizerError::from(("URI in addr-spec", part)).into())?
                        .1,
                    params,
                    phantom1: Default::default(),
                    phantom2: Default::default(),
                },
            ))
        }
    }
}

#[cfg(feature = "test-utils")]
impl testing_utils::Randomize for UriWithParams {
    fn random() -> Self {
        use testing_utils::Randomize;

        Self {
            uri: Randomize::random(),
            params: Randomize::rand_list0(3),
        }
    }
}
