use rsip::headers::header::max_forwards::{typed, Tokenizer};
use std::convert::TryInto;

//TODO: add automated tests for conversions
#[test]
fn typed() {
    assert_eq!(
        Tokenizer { value: "70" }.try_into(),
        Ok(typed::MaxForwards::new(70_u16))
    );
}

#[test]
fn tokenizer() {
    assert_eq!(Tokenizer::tokenize("70"), Ok(Tokenizer { value: "70" }));
}
