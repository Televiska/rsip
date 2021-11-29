use rsip::common::uri::param::{Maddr, Param, Tokenizer};
use std::convert::TryInto;

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(
            Param::Maddr(Maddr::new("255.255.255.0")).to_string(),
            String::from(";maddr=255.255.255.0")
        );
    }
}

mod parser {
    use super::*;

    #[test]
    fn parser1() {
        assert_eq!(
            Tokenizer::from(("maddr".as_bytes(), Some("255.255.255.0".as_bytes()))).try_into(),
            Ok(Param::Maddr(Maddr::new("255.255.255.0")))
        );
    }

    #[test]
    fn parser2() {
        assert_eq!(
            Tokenizer::from(("maddr".as_bytes(), None,)).try_into(),
            Ok(Param::Other("maddr".into(), None))
        );
    }
}

mod tokenizer {
    use super::*;

    #[test]
    fn tokenizer1_u8() {
        assert_eq!(
            Tokenizer::tokenize(";maddr=255.255.255.255;something".as_bytes()),
            Ok((
                ";something".as_bytes(),
                ("maddr".as_bytes(), Some("255.255.255.255".as_bytes())).into()
            )),
        );
    }

    #[test]
    fn tokenizer1_str() {
        assert_eq!(
            Tokenizer::tokenize(";maddr=255.255.255.255;something"),
            Ok((";something", ("maddr", Some("255.255.255.255")).into())),
        );
    }

    #[test]
    fn tokenizer2_u8() {
        assert_eq!(
            Tokenizer::tokenize(";maddr=255.255.255.255;something".as_bytes()),
            Ok((
                ";something".as_bytes(),
                ("maddr".as_bytes(), Some("255.255.255.255".as_bytes())).into()
            )),
        );
    }

    #[test]
    fn errors1() {
        assert_eq!(
            Tokenizer::tokenize("hello".as_bytes()),
            Err(nom::Err::Error(rsip::TokenizerError::from(
                "failed to tokenize uri param: hello"
            ))),
        );
    }
}
