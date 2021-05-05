use rsip::common::status_code::{StatusCode, Tokenizer};

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
        StatusCode::parse("200".as_bytes().into()),
        Ok(StatusCode::Ok)
    );

    assert_eq!(
        StatusCode::parse("700".as_bytes().into()),
        Ok(StatusCode::Other(700))
    );
}
