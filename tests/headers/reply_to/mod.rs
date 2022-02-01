#[cfg(feature = "test-utils")]
pub mod typed;

use rsip::headers::ReplyTo;

validate_untyped_header_trait!(ReplyTo);
validate_to_typed_header_trait!(ReplyTo);
