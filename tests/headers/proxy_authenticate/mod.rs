pub mod typed;

use rsip::headers::ProxyAuthenticate;

validate_untyped_header_trait!(ProxyAuthenticate);
validate_to_typed_header_trait!(ProxyAuthenticate);
