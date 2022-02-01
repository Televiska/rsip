use crate::{headers::typed::Tokenize, Error};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct CseqTokenizer<'a> {
    pub seq: &'a str,
    pub method: &'a str,
}

impl<'a> Tokenize<'a> for CseqTokenizer<'a> {
    fn tokenize(part: &'a str) -> Result<Self, Error> {
        use nom::{
            bytes::complete::take_until, character::complete::space1, combinator::rest,
            error::VerboseError, sequence::tuple,
        };

        let (_, (seq, _, method)) =
            tuple((take_until::<_, _, VerboseError<&str>>(" "), space1, rest))(part)
                .map_err(|_| Error::tokenizer(("cseq header", part)))?;

        Ok(Self { seq, method })
    }
}
