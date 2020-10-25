#[derive(Debug, PartialEq, Clone)]
pub struct MimeVersion(pub f32);

impl Eq for MimeVersion {}

impl Default for MimeVersion {
    fn default() -> Self {
        Self(1.0)
    }
}

impl Into<f32> for MimeVersion {
    fn into(self) -> f32 {
        self.0
    }
}

impl From<f32> for MimeVersion {
    fn from(from: f32) -> Self {
        Self(from)
    }
}

impl Into<libsip::headers::Header> for MimeVersion {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::MimeVersion(self.into())
    }
}
