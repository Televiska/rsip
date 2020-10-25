use crate::common::Method;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CSeq {
    pub seq: u32,
    pub method: Method,
}

impl Default for CSeq {
    fn default() -> Self {
        Self {
            seq: 1,
            method: Method::Register,
        }
    }
}

impl Into<libsip::headers::Header> for CSeq {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::CSeq(self.seq, self.method.into())
    }
}

impl From<(u32, Method)> for CSeq {
    fn from(tuple: (u32, Method)) -> Self {
        Self {
            seq: tuple.0,
            method: tuple.1,
        }
    }
}
