use crate::{
    common::{
        transport,
        uri::{self, param},
        version,
    },
    headers::typed::Tokenize,
};

#[derive(Eq, PartialEq, Debug)]
pub struct Tokenizer<'a> {
    pub version: version::Tokenizer<'a>,
    pub transport: transport::Tokenizer<'a>,
    pub uri: uri::Tokenizer<'a>,
    pub params: Vec<param::Tokenizer<'a>>,
}

impl<'a> Tokenize<'a> for Tokenizer<'a> {
    fn tokenize(part: &'a str) -> Result<Self, crate::Error> {
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
        ))(part.as_bytes())?;

        Ok(Self {
            version,
            transport,
            uri,
            params,
        })
    }
}
