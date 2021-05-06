use rsip::common::method::{Method, Tokenizer};
use std::convert::TryInto;

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"REGISTER something"),
        Ok(("something".as_bytes(), "REGISTER".as_bytes().into())),
    );
}

#[test]
fn parser() {
    assert_eq!(
        Tokenizer::from("REGISTER".as_bytes()).try_into(),
        Ok(Method::Register),
    );
}
