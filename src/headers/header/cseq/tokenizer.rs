use crate::headers::header::Tokenize;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Tokenizer<'a> {
    pub seq: &'a str,
    pub method: &'a str,
}

impl<'a> Tokenize<'a> for Tokenizer<'a> {
    fn tokenize(part: &'a str) -> Result<Self, crate::Error> {
        use nom::{
            bytes::complete::take_until, character::complete::space1, combinator::rest,
            error::VerboseError, sequence::tuple,
        };

        let (_, (seq, _, method)) =
            tuple((take_until::<_, _, VerboseError<&str>>(" "), space1, rest))(part)?;

        Ok(Self { seq, method })
    }
}

