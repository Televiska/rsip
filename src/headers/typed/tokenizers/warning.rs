use crate::{headers::typed::Tokenize, Error};

//trims spaces on each token as well
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct WarningTokenizer<'a> {
    pub code: &'a str,
    pub host: &'a str,
    pub text: &'a str,
}

impl<'a> From<(&'a str, &'a str, &'a str)> for WarningTokenizer<'a> {
    fn from((code, host, text): (&'a str, &'a str, &'a str)) -> Self {
        Self { code, host, text }
    }
}

impl<'a> Tokenize<'a> for WarningTokenizer<'a> {
    fn tokenize(part: &'a str) -> Result<Self, Error> {
        use nom::{
            bytes::complete::{tag, take_until},
            sequence::{terminated, tuple},
        };

        let (text, (code, host)) = tuple((
            terminated(take_until(" "), tag(" ")),
            terminated(take_until(" "), tag(" ")),
        ))(part)?;

        Ok(Self {
            code,
            host,
            text: text.trim_start_matches('"').trim_end_matches('"'),
        })
    }
}
