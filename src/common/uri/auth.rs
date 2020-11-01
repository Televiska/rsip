#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Auth {
    pub username: String,
    pub password: Option<String>,
}

impl Default for Auth {
    fn default() -> Self {
        Self {
            username: "foo@example.com".into(),
            password: Some("123123123".into()),
        }
    }
}

impl From<(String, Option<String>)> for Auth {
    fn from(tuple: (String, Option<String>)) -> Self {
        Self {
            username: tuple.0,
            password: tuple.1,
        }
    }
}

impl Into<libsip::uri::UriAuth> for Auth {
    fn into(self) -> libsip::uri::UriAuth {
        libsip::uri::UriAuth {
            username: self.username,
            password: self.password,
        }
    }
}

impl From<libsip::uri::UriAuth> for Auth {
    fn from(from: libsip::uri::UriAuth) -> Self {
        Self {
            username: from.username,
            password: from.password,
        }
    }
}
