#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Language {
    English,
    Custom(String),
}

impl Into<libsip::headers::Language> for Language {
    fn into(self) -> libsip::headers::Language {
        match self {
            Self::English => libsip::headers::Language::English,
            Self::Custom(_) => panic!("can't transform custom to libsip"),
        }
    }
}

impl From<libsip::headers::Language> for Language {
    fn from(from: libsip::headers::Language) -> Self {
        match from {
            libsip::headers::Language::English => Self::English,
            _ => Self::Custom(format!("{:?}", from)),
        }
    }
}
