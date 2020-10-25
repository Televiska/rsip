#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ContactParam {
    Custom(String, Option<GenValue>),
}

impl ContactParam {
    pub fn value(&self) -> Option<String> {
        match self {
            Self::Custom(_, gen_value) => gen_value.clone().map(Into::into),
        }
    }
}

//TODO: remove default from here
impl Default for ContactParam {
    fn default() -> Self {
        ContactParam::Custom("q".into(), Some(GenValue::Value("1.0".into())))
    }
}

impl Into<(String, Option<String>)> for ContactParam {
    fn into(self) -> (String, Option<String>) {
        match self {
            Self::Custom(key, gen_value) => (key, gen_value.map(|v| v.into())),
        }
    }
}

impl From<(String, Option<String>)> for ContactParam {
    fn from(tuple: (String, Option<String>)) -> Self {
        Self::Custom(tuple.0, tuple.1.map(Into::into))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GenValue {
    Value(String),
    QuotedValue(String),
}

impl Into<String> for GenValue {
    fn into(self) -> String {
        match self {
            Self::Value(value) => value,
            Self::QuotedValue(value) => format!("\"{}\"", value),
        }
    }
}

impl From<String> for GenValue {
    fn from(from: String) -> Self {
        if from.starts_with('\"') && from.ends_with('\"') {
            Self::QuotedValue(from)
        } else {
            Self::Value(from)
        }
    }
}

impl Into<(String, Option<libsip::headers::GenValue>)> for ContactParam {
    fn into(self) -> (String, Option<libsip::headers::GenValue>) {
        match self {
            Self::Custom(key, gen_value) => (key, gen_value.map(|v| v.into())),
        }
    }
}

impl From<(String, Option<libsip::headers::GenValue>)> for ContactParam {
    fn from(from: (String, Option<libsip::headers::GenValue>)) -> Self {
        Self::Custom(from.0, from.1.map(|s| s.into()))
    }
}

impl Into<libsip::headers::GenValue> for GenValue {
    fn into(self) -> libsip::headers::GenValue {
        match self {
            Self::Value(value) => libsip::headers::GenValue::Token(value),
            Self::QuotedValue(value) => libsip::headers::GenValue::QuotedString(value),
        }
    }
}

impl From<libsip::headers::GenValue> for GenValue {
    fn from(from: libsip::headers::GenValue) -> Self {
        match from {
            libsip::headers::GenValue::Token(value) => GenValue::Value(value),
            libsip::headers::GenValue::QuotedString(value) => GenValue::QuotedValue(value),
        }
    }
}
