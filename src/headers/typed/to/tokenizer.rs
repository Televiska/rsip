use crate::{common::uri, headers::typed::Tokenize, Error};

//TODO: this tokenizer is the same as From tokenizer, maybe merge?
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Tokenizer<'a> {
    pub display_name: Option<&'a str>,
    pub uri: uri::Tokenizer<'a>,
    pub params: Vec<uri::param::Tokenizer<'a>>,
}

impl<'a> Tokenize<'a> for Tokenizer<'a> {
    fn tokenize(part: &'a str) -> Result<Self, Error> {
        use nom::{
            bytes::complete::{tag, take_until},
            combinator::rest,
            error::VerboseError,
            multi::many0,
            sequence::tuple,
        };

        if part.contains('<') {
            let (_, (display_name, _, uri, _, params)) = tuple::<_, _, VerboseError<&str>, _>((
                take_until("<"),
                tag("<"),
                take_until(">"),
                tag(">"),
                rest,
            ))(part)
            .map_err(|_| Error::tokenizer(("to header", part)))?;

            Ok(Self {
                display_name: crate::utils::opt_trim(display_name),
                uri: uri::Tokenizer::tokenize(uri.as_bytes())
                    .map_err(|_| Error::tokenizer(("URI in name-addr of To header", part)))?
                    .1,
                params: many0(uri::param::Tokenizer::tokenize)(params.as_bytes())
                    .map_err(|_| Error::tokenizer(("params in name-addr of To header", part)))?
                    .1,
            })
        } else {
            let (_, (uri, params)) = tuple((
                uri::Tokenizer::tokenize_without_params,
                many0(uri::param::Tokenizer::tokenize),
            ))(part.as_bytes())?;

            Ok(Self {
                display_name: None,
                uri,
                params,
            })
        }
    }
}
