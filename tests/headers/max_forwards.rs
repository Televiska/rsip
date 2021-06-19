use rsip::headers::header::max_forwards::{self, Tokenizer};
use rsip::headers::header::Tokenize;
use std::convert::TryInto;

validate_untyped_header_trait!(max_forwards, MaxForwards);

#[test]
fn tokenizer() {
    assert_eq!(Tokenizer::tokenize("70"), Ok(Tokenizer { part: "70" }));
}

mod typed {
    use super::*;

    validate_typed_header_trait!(max_forwards, MaxForwards);

    #[test]
    fn typed() {
        assert_eq!(
            Tokenizer { part: "70" }.try_into(),
            Ok(max_forwards::typed::MaxForwards::new(70_u16))
        );
    }
}
