use crate::headers::Header;
use macros::{Display, FromIntoInner, FromStr, HasValue, IntoHeader, Typed};
use std::convert::TryInto;

pub use tokenizer::Tokenizer;

#[derive(
    HasValue, Display, IntoHeader, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone, Typed,
)]
pub struct MaxForwards(String);

/*
impl MaxForwards {
    pub fn typed(self) -> Result<typed::MaxForwards, Error> {
        self.try_into()
    }
}*/

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
    pub struct MaxForwards(u16);

    impl<'a> TryFrom<Tokenizer<'a>> for MaxForwards {
        type Error = crate::Error;

        fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
            Ok(MaxForwards(tokenizer.value.parse::<u16>()?))
        }
    }
}
