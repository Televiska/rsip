pub use crate::Error;
#[doc(hidden)]
pub use tokenizer::Tokenizer;

/// Simple struct that holds the authority part on of a URI.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Auth {
    pub user: String,
    pub password: Option<String>,
}

impl std::fmt::Display for Auth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.password {
            Some(password) => write!(f, "{}:{}", self.user, password),
            None => write!(f, "{}", self.user),
        }
    }
}

impl<T, S> From<(T, Option<S>)> for Auth
where
    T: Into<String>,
    S: Into<String>,
{
    fn from(from: (T, Option<S>)) -> Self {
        Self {
            user: from.0.into(),
            password: from.1.map(|p| p.into()),
        }
    }
}

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a str, char>> for Auth {
    type Error = Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a str, char>) -> Result<Self, Self::Error> {
        Ok(Auth {
            user: tokenizer.user.into(),
            password: tokenizer.password.map(Into::into),
        })
    }
}

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a [u8], u8>> for Auth {
    type Error = Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a [u8], u8>) -> Result<Self, Self::Error> {
        use std::str::from_utf8;

        Self::try_from(Tokenizer::from((
            from_utf8(tokenizer.user)?,
            tokenizer.password.map(from_utf8).transpose()?,
        )))
    }
}

#[doc(hidden)]
pub mod tokenizer {
    use crate::{AbstractInput, AbstractInputItem, GResult, GenericNomError, TokenizerError};
    use std::marker::PhantomData;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        pub user: T,
        pub password: Option<T>,
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
                user: from.0,
                password: from.1,
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
                bytes::complete::{tag, take_while},
                combinator::opt,
                sequence::tuple,
            };

            let (rem, user) =
                take_while(grammar_user)(part).map_err(|_: GenericNomError<'a, T>| {
                    TokenizerError::from(("auth user", part)).into()
                })?;

            let (rem, password) = opt(tuple((tag(":"), take_while(grammar_password))))(rem)
                .map_err(|_: GenericNomError<'a, T>| {
                    TokenizerError::from(("auth user password", part)).into()
                })?;

            let (rem, _) = tag("@")(rem).map_err(|_: GenericNomError<'a, T>| {
                TokenizerError::from(("auth user password at", part)).into()
            })?;

            let password = password.map(|(_tag, password)| password);

            Ok((rem, Tokenizer::from((user, password))))
        }
    }

    fn grammar_user<I: AbstractInputItem<I>>(c: I) -> bool {
        grammar_unreserved(&c) || grammar_escaped(&c) || grammar_user_unreserved(&c)
    }

    fn grammar_password<I: AbstractInputItem<I>>(c: I) -> bool {
        grammar_unreserved(&c) || grammar_escaped(&c) || grammar_password_unreserved(&c)
    }

    fn grammar_unreserved<I: AbstractInputItem<I>>(c: &I) -> bool {
        b"-_.!~*'()".iter().any(|mark| &I::from(*mark) == c) || c.clone().is_alphanum()
    }

    fn grammar_escaped<I: AbstractInputItem<I>>(c: &I) -> bool {
        &I::from(b'%') == c
    }

    fn grammar_user_unreserved<I: AbstractInputItem<I>>(c: &I) -> bool {
        b"&=+$,;?/"
            .iter()
            .any(|user_unreserved| &I::from(*user_unreserved) == c)
    }

    fn grammar_password_unreserved<I: AbstractInputItem<I>>(c: &I) -> bool {
        b"&=+$,"
            .iter()
            .any(|password_unreserved| &I::from(*password_unreserved) == c)
    }
}

#[cfg(feature = "test-utils")]
impl testing_utils::Randomize for Auth {
    fn random() -> Self {
        use testing_utils::{rand_str_of, sample};

        Self {
            user: rand_str_of(7),
            password: sample(&[Some(rand_str_of(10)), None]),
        }
    }
}
