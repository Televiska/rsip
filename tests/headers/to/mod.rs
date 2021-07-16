pub mod tokenizer;
pub mod typed;

use rsip::headers::To;

validate_untyped_header_trait!(To);
validate_to_typed_header_trait!(To);
