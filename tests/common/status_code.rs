use rsip::common::status_code::{StatusCode, Tokenizer};
use std::convert::TryInto;

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"200 something"),
        Ok(("something".as_bytes(), "200".as_bytes().into())),
    );
}

#[test]
fn parser() {
    assert_eq!(
        Tokenizer::from("200".as_bytes()).try_into(),
        Ok(StatusCode::Ok)
    );

    assert_eq!(
        Tokenizer::from("700".as_bytes()).try_into(),
        Ok(StatusCode::Other(700))
    );
}
