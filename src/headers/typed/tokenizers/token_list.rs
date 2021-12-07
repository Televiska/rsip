use crate::{headers::typed::Tokenize, Error};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct TokenListTokenizer<'a> {
    pub tokens: Vec<&'a str>,
}

impl<'a> Tokenize<'a> for TokenListTokenizer<'a> {
    fn tokenize(part: &'a str) -> Result<Self, Error> {
        use nom::{
            bytes::complete::{tag, take_until},
            multi::many0,
            sequence::terminated,
        };

        let stopbreak = terminated(take_until(","), tag(","));

        let (last_token, mut tokens) = many0(stopbreak)(part)?;

        tokens.push(last_token);

        Ok(Self { tokens })
    }
}
