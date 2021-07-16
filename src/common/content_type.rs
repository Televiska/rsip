use rsip_derives::NewType;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ContentType {
    Sdp,
    Other(OtherContentType),
}

#[derive(NewType, Debug, PartialEq, Eq, Clone)]
pub struct OtherContentType(String);
