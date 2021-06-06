use rsip::headers::header::content_length::{self, Tokenizer};
use rsip::headers::header::Tokenize;
use std::convert::TryInto;

validate_untyped_header_trait!(content_length, ContentLength);

#[test]
fn tokenizer() {
    assert_eq!(Tokenizer::tokenize("70"), Ok(Tokenizer { part: "70" }));
}

mod typed {
    use super::*;

    validate_typed_header_trait!(content_length, ContentLength);

    #[test]
    fn typed() {
        assert_eq!(
            Tokenizer { part: "70" }.try_into(),
            Ok(content_length::typed::ContentLength::new(70_u16))
        );
    }
}
