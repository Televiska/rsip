#[doc(hidden)]
pub use super::tokenizers::NameParamsTokenizer as Tokenizer;

use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MediaType {
    Sdp(Vec<MediaTypeParam>),
    Other(String, Vec<MediaTypeParam>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MediaTypeParam(pub String, pub String);

impl<S, T> From<(S, T)> for MediaTypeParam
where
    S: std::fmt::Display,
    T: std::fmt::Display,
{
    fn from(tuple: (S, T)) -> Self {
        Self(tuple.0.to_string(), tuple.1.to_string())
    }
}

impl<'a> TryFrom<Tokenizer<'a>> for MediaType {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer) -> Result<Self, Self::Error> {
        match tokenizer.name {
            s if s.eq_ignore_ascii_case("application/sdp") => Ok(Self::Sdp(
                tokenizer
                    .params
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<_>>(),
            )),
            _ => Ok(Self::Other(
                tokenizer.name.to_string(),
                tokenizer
                    .params
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<_>>(),
            )),
        }
    }
}

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sdp(params) if params.is_empty() => write!(f, "application/sdp"),
            Self::Sdp(params) => write!(f, "application/sdp; {}", params_to_string(params)),
            Self::Other(name, params) if params.is_empty() => write!(f, "{}", name),
            Self::Other(name, params) => write!(f, "{}; {}", name, params_to_string(params)),
        }
    }
}

impl std::fmt::Display for MediaTypeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.0, self.1)
    }
}

fn params_to_string(params: &[MediaTypeParam]) -> String {
    params
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join("; ")
}
