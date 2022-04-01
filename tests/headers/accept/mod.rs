pub mod typed;

use rsip::headers::Accept;

validate_untyped_header_trait!(Accept);
validate_to_typed_header_trait!(Accept);
