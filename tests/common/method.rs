use rsip::common::method::{Method, Tokenizer};
use std::convert::TryInto;

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(Method::Register.to_string(), String::from("REGISTER"));
    }
}

mod parser {
    use super::*;

    #[test]
    fn parser1() {
        assert_eq!(
            Tokenizer::from("REGISTER".as_bytes()).try_into(),
            Ok(Method::Register),
        );
    }

    #[test]
    fn errors1() {
        use std::convert::TryFrom;

        assert_eq!(
            Method::try_from(Tokenizer::from("REGISTE".as_bytes())),
            Err(rsip::Error::ParseError("invalid method: REGISTE".into())),
        );
    }
}

mod tokenizer {
    use super::*;

    #[test]
    fn tokenizer1_u8() {
        assert_eq!(
            Tokenizer::tokenize("REGISTER something".as_bytes()),
            Ok((" something".as_bytes(), "REGISTER".as_bytes().into())),
        );
    }

    #[test]
    fn tokenizer1_str() {
        assert_eq!(
            Tokenizer::tokenize("REGISTER something"),
            Ok((" something", "REGISTER".into())),
        );
    }

    #[test]
    fn errors1() {
        assert_eq!(
            Tokenizer::tokenize("<<< something".as_bytes()),
            Err(nom::Err::Error(rsip::TokenizerError::from(
                "failed to tokenize method: <<< something"
            ))),
        );
    }
}
