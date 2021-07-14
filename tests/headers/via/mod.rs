pub mod tokenizer;
pub mod typed;

use rsip::headers::Via;

validate_untyped_header_trait!(Via);
validate_to_typed_header_trait!(Via);
