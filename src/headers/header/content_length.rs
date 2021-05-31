use crate::headers::Header;
use macros::{Display, FromIntoInner, FromStr, HasValue, IntoHeader, Typed};
use std::convert::TryInto;

pub use tokenizer::Tokenizer;

#[derive(
    Typed, HasValue, Display, IntoHeader, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone,
)]
pub struct ContentLength(String);

pub mod tokenizer {
    #[derive(macros::FromValue, Eq, PartialEq, Clone, Debug)]
    pub struct Tokenizer<'a> {
        pub value: &'a str,
    }

    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a str) -> Result<Self, crate::Error> {
            Ok(Self { value: part })
        }
    }
}

pub mod typed {
    use super::Tokenizer;
    use macros::{FromUntyped, HasValue, ValueDisplay};
    use std::convert::{TryFrom, TryInto};

    #[derive(HasValue, ValueDisplay, FromUntyped, Eq, PartialEq, Clone, Debug)]
    pub struct ContentLength(u32);

    impl<'a> TryFrom<Tokenizer<'a>> for ContentLength {
        type Error = crate::Error;

        fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
            Ok(ContentLength(tokenizer.value.parse::<u32>()?))
        }
    }
}
