pub mod typed;

use rsip::headers::ProxyAuthorization;

validate_untyped_header_trait!(ProxyAuthorization);
validate_to_typed_header_trait!(ProxyAuthorization);
