use rsip_derives::{IntoParam, NewType};

/// Simple NewType around String. Intended to be used for the `user` parameter found in SIP(S)
/// uris, rarely used nowardays.
#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct User(String);

#[cfg(feature = "test-utils")]
impl testing_utils::Randomize for User {
    fn random() -> Self {
        Self(testing_utils::sample(&["phone", "ip", &testing_utils::rand_str_of(5)]).to_string())
    }
}
