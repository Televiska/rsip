#[doc(hidden)]
pub use super::tokenizers::NameParamsTokenizer as Tokenizer;

use rsip_derives::TypedHeader;
use std::convert::TryFrom;

/// The `Content-Disposition` header in its [typed](super) form.
#[derive(TypedHeader, Eq, PartialEq, Clone, Debug)]
pub struct ContentDisposition {
    pub display_type: DisplayType,
    pub display_params: Vec<DisplayTypeParam>,
}

impl ContentDisposition {
    pub fn is_session(&self) -> bool {
        matches!(self.display_type, DisplayType::Session)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DisplayType {
    Render,
    Session,
    Icon,
    Alert,
    Other(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DisplayTypeParam(pub String, pub String);

impl<S, T> From<(S, T)> for DisplayTypeParam
where
    S: std::fmt::Display,
    T: std::fmt::Display,
{
    fn from(tuple: (S, T)) -> Self {
        Self(tuple.0.to_string(), tuple.1.to_string())
    }
}

impl<'a> TryFrom<Tokenizer<'a>> for ContentDisposition {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        let display_type = match tokenizer.name {
            s if s.eq_ignore_ascii_case("render") => DisplayType::Render,
            s if s.eq_ignore_ascii_case("session") => DisplayType::Session,
            s if s.eq_ignore_ascii_case("icon") => DisplayType::Icon,
            s if s.eq_ignore_ascii_case("alert") => DisplayType::Alert,
            s => DisplayType::Other(s.into()),
        };

        Ok(Self {
            display_type,
            display_params: tokenizer
                .params
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>(),
        })
    }
}

impl std::fmt::Display for DisplayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Render => write!(f, "render"),
            Self::Session => write!(f, "session"),
            Self::Icon => write!(f, "icon"),
            Self::Alert => write!(f, "alert"),
            Self::Other(other) => write!(f, "{}", other),
        }
    }
}

impl std::fmt::Display for DisplayTypeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.0, self.1)
    }
}

impl std::fmt::Display for ContentDisposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.display_params.is_empty() {
            write!(f, "{}", self.display_type)
        } else {
            write!(
                f,
                "{}; {}",
                self.display_type,
                self.display_params
                    .iter()
                    .map(|param| param.to_string())
                    .collect::<Vec<String>>()
                    .join("; ")
            )
        }
    }
}
