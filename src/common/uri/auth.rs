use crate::Error;
use nom::error::VerboseError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Auth {
    pub username: String,
    pub password: Option<String>,
}

impl From<(String, Option<String>)> for Auth {
    fn from(tuple: (String, Option<String>)) -> Self {
        Self {
            username: tuple.0,
            password: tuple.1,
        }
    }
}

impl Auth {
    pub fn parse(tokenizer: Tokenizer) -> Result<Self, Error> {
        use std::str::from_utf8;

        Ok(Self {
            username: from_utf8(tokenizer.username)?.into(),
            password: tokenizer
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
    pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), nom::Err<VerboseError<&'a [u8]>>> {
        use nom::{
            branch::alt,
            bytes::complete::{tag, take_until},
            combinator::rest,
            sequence::tuple,
        };

        let (rem, (auth, _)) = tuple((alt((take_until("@"), take_until("."))), tag("@")))(part)?;
        let (username, password) =
            match tuple::<_, _, VerboseError<&'a [u8]>, _>((take_until(":"), tag(":"), rest))(auth)
            {
                Ok((_, (username, _, password))) => (username, Some(password)),
                Err(_) => {
                    let (_, username) = rest(auth)?;
                    (username, None)
                }
            };

        Ok((rem, Tokenizer { username, password }))
    }
}
