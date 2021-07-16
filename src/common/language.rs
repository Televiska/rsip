use rsip_derives::NewType;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Language {
    English,
    Other(OtherLanguage),
}

#[derive(NewType, Debug, PartialEq, Eq, Clone)]
pub struct OtherLanguage(String);
