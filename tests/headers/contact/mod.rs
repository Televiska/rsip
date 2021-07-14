pub mod tokenizer;
pub mod typed;

use rsip::headers::Contact;

validate_untyped_header_trait!(Contact);
validate_to_typed_header_trait!(Contact);
