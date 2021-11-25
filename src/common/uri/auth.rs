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

#[doc(hidden)]
pub mod tokenizer {
    use super::Auth;
    use crate::{Error, IResult, NomError, TokenizerError};
    use std::convert::TryInto;

    impl<'a> TryInto<Auth> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<Auth, Error> {
            use std::str::from_utf8;

            Ok(Auth {
                user: from_utf8(self.user)?.into(),
                password: self
                    .password
                    .map(|p| from_utf8(p))
                    .transpose()?
                    .map(Into::into),
            })
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Tokenizer<'a> {
        pub user: &'a [u8],
        pub password: Option<&'a [u8]>,
    }

    impl<'a> From<(&'a [u8], Option<&'a [u8]>)> for Tokenizer<'a> {
        fn from(value: (&'a [u8], Option<&'a [u8]>)) -> Self {
            Self {
                user: value.0,
                password: value.1,
            }
        }
    }

    #[allow(clippy::type_complexity)]
    impl<'a> Tokenizer<'a> {
        //we alt with take_until(".") and then tag("@") to make sure we fail early
        pub fn tokenize(part: &'a [u8]) -> IResult<Self> {
            use nom::{
                bytes::complete::{tag, take_till, take_until},
                combinator::rest,
                error::VerboseError,
                sequence::tuple,
            };

            let (rem, (auth, _)) =
                tuple((take_till(|c| c == b'.' || c == b'@'), tag("@")))(part)
                    .map_err(|_: NomError<'a>| TokenizerError::from(("auth user", part)).into())?;

            let (user, password) =
                match tuple::<_, _, VerboseError<&'a [u8]>, _>((take_until(":"), tag(":"), rest))(
                    auth,
                ) {
                    Ok((_, (user, _, password))) => (user, Some(password)),
                    Err(_) => {
                        //this is not going to ever fail actually, since rest never returns an
                        //error
                        let (_, user) = rest(auth).map_err(|_: crate::NomError<'a>| {
                            TokenizerError::from(("auth user (no password)", auth)).into()
                        })?;
                        (user, None)
                    }
                };

            Ok((rem, Tokenizer { user, password }))
        }
    }
}
