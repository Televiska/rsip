#[cfg(feature = "test-utils")]
pub mod typed;

use rsip::headers::ErrorInfo;

validate_untyped_header_trait!(ErrorInfo);
validate_to_typed_header_trait!(ErrorInfo);
