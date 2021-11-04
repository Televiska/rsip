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

pub mod tokenizer {
    use rsip::common::status_code::Tokenizer;

    #[test]
    fn with_str_input() {
        assert_eq!(
            Tokenizer::tokenize("200 OK\r\nsomething"),
            Ok(("\r\nsomething", ("200", "OK").into())),
        );
    }

    #[test]
    fn with_bytes_input() {
        assert_eq!(
            Tokenizer::tokenize("200 OK\r\nsomething".as_bytes()),
            Ok((
                "\r\nsomething".as_bytes(),
                ("200".as_bytes(), "OK".as_bytes()).into()
            )),
        );
    }
}
