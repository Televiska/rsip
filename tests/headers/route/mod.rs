#[cfg(feature = "test-utils")]
pub mod typed;

use rsip::headers::RecordRoute;

validate_untyped_header_trait!(RecordRoute);
validate_to_typed_header_trait!(RecordRoute);
