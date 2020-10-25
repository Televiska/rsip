#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NamedParam {
    Tag(Tag),
    Custom(String, Option<String>),
}

//TODO: remove default from here
impl Default for NamedParam {
    fn default() -> Self {
        Self::Tag(Default::default())
    }
}

impl Into<(String, Option<String>)> for NamedParam {
    fn into(self) -> (String, Option<String>) {
        match self {
            Self::Tag(tag) => ("tag".into(), Some(tag.into())),
            Self::Custom(key, value) => (key, value),
        }
    }
}

impl From<(String, Option<String>)> for NamedParam {
    fn from(tuple: (String, Option<String>)) -> Self {
        match tuple {
            (key, value) if key.trim().eq_ignore_ascii_case("tag") && value.is_some() => {
                Self::Tag(value.unwrap().into())
            }
            (key, value) => Self::Custom(key, value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tag(String);

impl Default for Tag {
    fn default() -> Self {
        Self("arandomtag".into())
    }
}

impl Into<String> for Tag {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for Tag {
    fn from(from: String) -> Self {
        Self(from)
    }
}
