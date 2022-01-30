pub mod typed;

use rsip::headers::ContentType;

validate_untyped_header_trait!(ContentType);
validate_to_typed_header_trait!(ContentType);
