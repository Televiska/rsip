use rsip::headers::header::content_length::{typed, Tokenizer};
use std::convert::TryInto;

#[test]
fn typed() {
    assert_eq!(
        Tokenizer { value: "70" }.try_into(),
        Ok(typed::ContentLength::new(70_u16))
    );
}

#[test]
fn tokenizer() {
    assert_eq!(Tokenizer::tokenize("70"), Ok(Tokenizer { value: "70" }));
}
