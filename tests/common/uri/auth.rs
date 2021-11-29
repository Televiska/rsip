use rsip::common::uri::auth::{Auth, Tokenizer};
use std::convert::TryInto;

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(
            Auth {
                user: "user".into(),
                password: None
            }
            .to_string(),
            String::from("user")
        );
    }

    #[test]
    fn display2() {
        assert_eq!(
            Auth {
                user: "user".into(),
                password: Some("password".into())
            }
            .to_string(),
            String::from("user:password")
        );
    }
}

mod parser {
    use super::*;

    #[test]
    fn parser() {
        assert_eq!(
            Tokenizer::from(("user".as_bytes(), Some("password".as_bytes()))).try_into(),
            Ok(Auth {
                user: "user".into(),
                password: Some("password".into())
            }),
        );
    }
}

mod tokenizer {
    use super::*;

    #[test]
    fn tokenizer1_u8() {
        assert_eq!(
            Tokenizer::tokenize("user:password@server2.com something".as_bytes()),
            Ok((
                "server2.com something".as_bytes(),
                ("user".as_bytes(), Some("password".as_bytes())).into()
            )),
        );
    }

    #[test]
    fn tokenizer1_str() {
        assert_eq!(
            Tokenizer::tokenize("user:password@server2.com something"),
            Ok(("server2.com something", ("user", Some("password")).into())),
        );
    }

    #[test]
    fn errors1() {
        assert_eq!(
            Tokenizer::tokenize("server2.com something".as_bytes()),
            Err(nom::Err::Error(rsip::TokenizerError::from(
                "failed to tokenize auth user: server2.com something"
            ))),
        );
    }
}
