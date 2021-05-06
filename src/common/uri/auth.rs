pub use tokenizer::Tokenizer;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Auth {
    pub username: String,
    pub password: Option<String>,
}

impl<T, S> From<(T, Option<S>)> for Auth
where
    T: Into<String>,
    S: Into<String>,
{
    fn from(from: (T, Option<S>)) -> Self {
        Self {
            username: from.0.into(),
            password: from.1.map(|p| p.into()),
        }
    }
}

pub mod tokenizer {
    use super::Auth;
    use crate::{Error, NomError};
    use nom::error::VerboseError;
    use std::convert::TryInto;

    impl<'a> TryInto<Auth> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<Auth, Error> {
            use std::str::from_utf8;

            Ok(Auth {
                username: from_utf8(self.username)?.into(),
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
        pub username: &'a [u8],
        pub password: Option<&'a [u8]>,
    }

    impl<'a> From<(&'a [u8], Option<&'a [u8]>)> for Tokenizer<'a> {
        fn from(value: (&'a [u8], Option<&'a [u8]>)) -> Self {
            Self {
                username: value.0,
                password: value.1,
            }
        }
    }

    #[allow(clippy::type_complexity)]
    impl<'a> Tokenizer<'a> {
        //we alt with take_until(".") and then tag("@") to make sure we fail early
        pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
            use nom::{
                bytes::complete::{tag, take_till, take_until},
                combinator::rest,
                sequence::tuple,
            };
            let (rem, (auth, _)) = tuple((take_till(|c| c == b'.' || c == b'@'), tag("@")))(part)?;

            let (username, password) =
                match tuple::<_, _, VerboseError<&'a [u8]>, _>((take_until(":"), tag(":"), rest))(
                    auth,
                ) {
                    Ok((_, (username, _, password))) => (username, Some(password)),
                    Err(_) => {
                        let (_, username) = rest(auth)?;
                        (username, None)
                    }
                };

            Ok((rem, Tokenizer { username, password }))
        }
    }
}
