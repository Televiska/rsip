pub mod typed;

use rsip::headers::AuthenticationInfo;

validate_untyped_header_trait!(AuthenticationInfo);
validate_to_typed_header_trait!(AuthenticationInfo);
