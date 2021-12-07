pub mod typed;

use rsip::headers::Authorization;

validate_untyped_header_trait!(Authorization);
validate_to_typed_header_trait!(Authorization);
