pub mod tokenizer;
pub mod typed;

use rsip::headers::CSeq;

validate_untyped_header_trait!(CSeq);
validate_to_typed_header_trait!(CSeq);
