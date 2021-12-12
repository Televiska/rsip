#[cfg(feature = "test-utils")]
pub mod typed;

use rsip::headers::AlertInfo;

validate_untyped_header_trait!(AlertInfo);
validate_to_typed_header_trait!(AlertInfo);
