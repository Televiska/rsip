pub mod typed;

use rsip::headers::ContentDisposition;

validate_untyped_header_trait!(ContentDisposition);
validate_to_typed_header_trait!(ContentDisposition);
