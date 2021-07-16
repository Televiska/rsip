pub mod tokenizer;
pub mod typed;

use rsip::headers::WwwAuthenticate;

validate_untyped_header_trait!(WwwAuthenticate);
validate_to_typed_header_trait!(WwwAuthenticate);
