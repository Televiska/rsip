use crate::{
    common::{
        transport,
        uri::{self, param},
        version,
    },
    headers::typed::Tokenize,
    Error,
};

#[derive(Eq, PartialEq, Debug)]
pub struct Tokenizer<'a> {
    pub version: version::Tokenizer<'a, &'a str, char>,
    pub transport: transport::Tokenizer<'a, &'a str, char>,
    pub uri: uri::Tokenizer<'a, &'a str, char>,
    pub params: Vec<uri::param::Tokenizer<'a, &'a str, char>>,
}

impl<'a> Tokenize<'a> for Tokenizer<'a> {
    fn tokenize(part: &'a str) -> Result<Self, Error> {
        use nom::{
            bytes::complete::tag, character::complete::space1, multi::many0, sequence::tuple,
        };

        let (_, (version, _, transport, _, uri, params)) = tuple((
            version::Tokenizer::tokenize,
            tag("/"),
            transport::Tokenizer::tokenize,
            space1,
            uri::Tokenizer::tokenize_without_params,
            many0(param::Tokenizer::tokenize),
        ))(part)
        .map_err(|_| Error::tokenizer(("via (typed) header", part)))?;

        Ok(Self {
            version,
            transport,
            uri,
            params,
        })
    }
}
