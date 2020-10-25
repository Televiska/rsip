#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Version {
    V1,
    V2,
    //Custom(String)
}

impl Default for Version {
    fn default() -> Self {
        Self::V2
    }
}

impl Into<libsip::Version> for Version {
    fn into(self) -> libsip::Version {
        match self {
            Self::V1 => libsip::Version::new(1, 0),
            Self::V2 => libsip::Version::new(2, 0),
        }
    }
}

impl From<libsip::Version> for Version {
    fn from(from: libsip::Version) -> Self {
        match from.to_string().as_str() {
            "SIP/1.0" => Self::V1,
            "SIP/2.0" => Self::V2,
            _ => panic!("can't convert sane version from libsip!"),
        }
    }
}
