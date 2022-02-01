pub mod typed;

use rsip::headers::Warning;

validate_untyped_header_trait!(Warning);
validate_to_typed_header_trait!(Warning);
