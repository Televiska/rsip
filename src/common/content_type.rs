#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ContentType {
    Sdp,
    Custom(String),
}

impl Into<libsip::headers::ContentType> for ContentType {
    fn into(self) -> libsip::headers::ContentType {
        match self {
            Self::Sdp => libsip::headers::ContentType::Sdp,
            Self::Custom(_) => panic!("can't transform custom to libsip"),
        }
    }
}

impl From<libsip::headers::ContentType> for ContentType {
    fn from(from: libsip::headers::ContentType) -> Self {
        match from {
            libsip::headers::ContentType::Sdp => Self::Sdp,
            _ => Self::Custom(format!("{:?}", from)),
        }
    }
}
