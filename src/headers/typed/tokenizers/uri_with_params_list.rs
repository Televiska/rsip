use crate::{headers::typed::Tokenize, uri_with_params_list, Error};

pub type UriWithParamsListTokenizer<'a> = uri_with_params_list::Tokenizer<'a, &'a str, char>;

impl<'a> Tokenize<'a> for UriWithParamsListTokenizer<'a> {
    fn tokenize(part: &'a str) -> Result<Self, Error> {
        Ok(Self::tokenize(part)?.1)
    }
}
