use crate::{common::auth, headers::typed::Tokenize};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Tokenizer<'a> {
    pub scheme: auth::scheme::Tokenizer<'a, &'a str>,
    pub params: Vec<(&'a str, &'a str)>,
}

impl<'a> Tokenize<'a> for Tokenizer<'a> {
    fn tokenize(part: &'a str) -> Result<Self, crate::Error> {
        use nom::{
            branch::alt,
            bytes::complete::{tag, take_until},
            character::complete::space0,
            combinator::{map, opt, rest},
            error::VerboseError,
            multi::many1,
            sequence::{delimited, tuple},
        };

        let params = map::<_, _, _, VerboseError<&str>, _, _>(
            tuple((
                space0,
                take_until("="),
                tag("="),
                alt((
                    delimited(tag("\""), take_until("\""), tag("\"")),
                    take_until(","),
                    take_until(" "),
                    rest,
                )),
                opt(tag(",")),
            )),
            |tuple| (tuple.1, tuple.3),
        );
        let (_, (_, scheme, params)) =
            tuple((space0, auth::scheme::Tokenizer::tokenize, many1(params)))(part)?;

        let params = params
            .into_iter()
            .map(|(key, value)| {
                if value.starts_with('"') && value.ends_with('"') {
                    (key, &value[1..(value.len() - 1)])
                } else {
                    (key, value)
                }
            })
            .collect::<Vec<_>>();

        Ok(Self { scheme, params })
    }
}
