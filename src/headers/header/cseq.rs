use macros::UntypedHeader;

pub use tokenizer::Tokenizer;

#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
#[header(display_name = "CSeq")]
pub struct CSeq(String);

pub mod tokenizer {
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
}

pub mod typed {
    use super::Tokenizer;
    use crate::common::Method;
    use macros::TypedHeader;
    use std::convert::TryFrom;

    #[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
    pub struct CSeq {
        pub seq: u16,
        pub method: Method,
    }

    impl From<(u16, Method)> for CSeq {
        fn from(tuple: (u16, Method)) -> Self {
            Self {
                seq: tuple.0,
                method: tuple.1,
            }
        }
    }

    impl<'a> TryFrom<Tokenizer<'a>> for CSeq {
        type Error = crate::Error;

        fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
            Ok(CSeq {
                seq: tokenizer.seq.parse::<u16>()?,
                method: tokenizer.method.parse::<Method>()?,
            })
        }
    }

    impl std::fmt::Display for CSeq {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {}", self.seq, self.method)
        }
    }
}
