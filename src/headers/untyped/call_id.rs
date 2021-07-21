use rsip_derives::UntypedHeader;
use uuid::Uuid;

/// The `Call-ID` header in its [untyped](super) form.
#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
#[header(display_name = "Call-ID")]
pub struct CallId(String);

impl Default for CallId {
    fn default() -> Self {
        Self(format!("{}@example.com", Uuid::new_v4().to_string()))
    }
}
