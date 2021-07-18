use rsip_derives::{IntoParam, NewType};

/// Simple NewType around String. Intended to be used for the `method` parameter found in SIP(S)
/// uris, rarely used nowardays.
#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct Method(String);
