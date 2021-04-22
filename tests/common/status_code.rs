use rsip::common::status_code::{StatusCode, Tokenizer};

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"200 something"),
        Ok((b"something".as_ref(), b"200".as_ref().into())),
    );
}

#[test]
fn parser() {
    assert_eq!(
        StatusCode::parse(b"200".as_ref().into()),
        Ok(StatusCode::OK)
    );

    assert_eq!(
        StatusCode::parse(b"700".as_ref().into()),
        Ok(StatusCode::Other(700))
    );
}
