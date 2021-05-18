use crate::headers::Header;
use macros::{Display, FromIntoInner, FromStr, HasValue, IntoHeader, Typed};
use std::convert::TryInto;

pub use tokenizer::Tokenizer;

#[derive(
    HasValue, Display, IntoHeader, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone, Typed,
)]
pub struct CSeq(String);

pub mod tokenizer {
    #[derive(Eq, PartialEq, Clone, Debug)]
    pub struct Tokenizer<'a> {
        pub seq: &'a str,
        pub method: &'a str,
    }

    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a str) -> Result<Self, crate::Error> {
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
    use macros::FromUntyped;
    use std::convert::{TryFrom, TryInto};

    #[derive(FromUntyped, Eq, PartialEq, Clone, Debug)]
    pub struct CSeq {
        pub seq: u16,
        pub method: Method,
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
}
