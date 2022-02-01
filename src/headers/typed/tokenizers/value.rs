use crate::{headers::typed::Tokenize, Error};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ValueTokenizer<'a> {
    pub value: &'a str,
}

impl<'a> From<&'a str> for ValueTokenizer<'a> {
    fn from(value: &'a str) -> Self {
        Self { value }
    }
}

impl<'a> Tokenize<'a> for ValueTokenizer<'a> {
    fn tokenize(value: &'a str) -> Result<Self, Error> {
        Ok(Self { value })
    }
}
