#[cfg(feature = "test-utils")]
pub mod typed;

use rsip::headers::CallInfo;

validate_untyped_header_trait!(CallInfo);
validate_to_typed_header_trait!(CallInfo);
