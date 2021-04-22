use macros::{Display, FromIntoInner, FromStr, HasValue};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ContentType {
    Sdp,
    Other(OtherContentType),
}

#[derive(HasValue, Display, FromStr, FromIntoInner, Debug, PartialEq, Eq, Clone)]
pub struct OtherContentType(String);
