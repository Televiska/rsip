pub mod typed;

use rsip::headers::Allow;

validate_untyped_header_trait!(Allow);
validate_to_typed_header_trait!(Allow);
