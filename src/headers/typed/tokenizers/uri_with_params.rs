use crate::{headers::typed::Tokenize, uri_with_params, Error};

pub type UriWithParamsTokenizer<'a> = uri_with_params::Tokenizer<'a, &'a str, char>;

impl<'a> Tokenize<'a> for UriWithParamsTokenizer<'a> {
    fn tokenize(part: &'a str) -> Result<Self, Error> {
        Ok(Self::tokenize(part)?.1)
    }
}
