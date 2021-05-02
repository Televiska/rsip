use crate::headers::Header;
use macros::{Display, FromIntoInner, FromStr, HasValue, IntoHeader};

#[derive(HasValue, Display, IntoHeader, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct Accept(String);

/*
impl Accept {
    pub fn parse(tokenizer: header::Tokenizer) -> Result<Self, Error> {
        use std::str::from_utf8;

        Ok(Self(from_utf8(tokenizer.value)?.into()))
    }
}*/
