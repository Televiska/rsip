use rsip::common::status_code::{StatusCode, Tokenizer};
use std::convert::TryInto;

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(StatusCode::OK.to_string(), String::from("200 OK"));
    }

    #[test]
    fn display2() {
        assert_eq!(
            StatusCode::Other(700, "Something".into()).to_string(),
            String::from("700 Something")
        );
    }
}

mod parser {
    use super::*;

    #[test]
    fn parser1() {
        assert_eq!(
            Tokenizer::from(("200".as_bytes(), "OK".as_bytes())).try_into(),
            Ok(StatusCode::OK)
        );
    }

    #[test]
    fn parser2() {
        assert_eq!(
            Tokenizer::from(("200".as_bytes(), "NOTOK".as_bytes())).try_into(),
            Ok(StatusCode::OK)
        );
    }

    #[test]
    fn parser3() {
        assert_eq!(
            Tokenizer::from(("700".as_bytes(), "Something".as_bytes())).try_into(),
            Ok(StatusCode::Other(700, "Something".into()))
        );
    }
}

pub mod tokenizer {
    use super::*;

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

    #[test]
    fn errors1() {
        assert_eq!(
            Tokenizer::tokenize("12 OK\r\n".as_bytes()),
            Err(nom::Err::Error(rsip::TokenizerError::from(
                "failed to tokenize status: 12 OK\r\n"
            ))),
        );
    }
}
