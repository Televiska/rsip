use macros::{Display, FromIntoInner, FromStr, HasValue};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Language {
    English,
    Other(OtherLanguage),
}

#[derive(HasValue, Display, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct OtherLanguage(String);
