pub mod typed;

//use rsip::headers::{typed, From, UntypedHeader, ToTypedHeader};
use rsip::headers::From;

validate_untyped_header_trait!(From);
validate_to_typed_header_trait!(From);
