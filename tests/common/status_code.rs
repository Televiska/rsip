use rsip::common::status_code::{StatusCode, Tokenizer};
use std::convert::TryInto;

#[test]
fn display() {
    assert_eq!(StatusCode::OK.to_string(), String::from("200 OK"));

    assert_eq!(
        StatusCode::Other(700, "Something".into()).to_string(),
        String::from("700 Something")
    );
}

#[test]
fn parser() {
    assert_eq!(
        Tokenizer::from(("200".as_bytes(), "OK".as_bytes())).try_into(),
        Ok(StatusCode::OK)
    );

    assert_eq!(
        Tokenizer::from(("200".as_bytes(), "NOTOK".as_bytes())).try_into(),
        Ok(StatusCode::OK)
    );

    assert_eq!(
        Tokenizer::from(("700".as_bytes(), "Something".as_bytes())).try_into(),
        Ok(StatusCode::Other(700, "Something".into()))
    );
}

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"200 OK\r\nsomething"),
        Ok((
            "something".as_bytes(),
            ("200".as_bytes(), "OK".as_bytes()).into()
        )),
    );
}
