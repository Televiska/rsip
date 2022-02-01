pub mod typed;

use rsip::headers::Priority;

validate_untyped_header_trait!(Priority);
validate_to_typed_header_trait!(Priority);
