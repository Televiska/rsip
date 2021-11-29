use crate::{common::uri, headers::typed::Tokenize, Error};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Tokenizer<'a> {
    pub display_name: Option<&'a str>,
    pub uri: uri::Tokenizer<'a, &'a str, char>,
    pub params: Vec<uri::param::Tokenizer<'a, &'a str, char>>,
}

impl<'a> Tokenize<'a> for Tokenizer<'a> {
    fn tokenize(part: &'a str) -> Result<Self, Error> {
        use crate::parser_utils::is_empty_or_fail_with;
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
            .map_err(|_| Error::tokenizer(("contact header", part)))?;

            let (rem, params) = many0(uri::param::Tokenizer::tokenize)(params)
                .map_err(|_| Error::tokenizer(("params of Contact header", part)))?;
            is_empty_or_fail_with(rem, ("contact params", part))?;

            Ok(Self {
                display_name: crate::utils::opt_trim(display_name),
                uri: uri::Tokenizer::tokenize(uri)
                    .map_err(|_| Error::tokenizer(("URI in Contact header", part)))?
                    .1,
                params,
            })
        } else {
            let (_, (uri, params)) = tuple((
                uri::Tokenizer::tokenize_without_params,
                many0(uri::param::Tokenizer::tokenize),
            ))(part)?;

            Ok(Self {
                display_name: None,
                uri,
                params,
            })
        }
    }
}
