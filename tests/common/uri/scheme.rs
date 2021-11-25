use rsip::common::uri::scheme::{Scheme, Tokenizer};
use std::convert::TryInto;

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(Scheme::Sip.to_string(), String::from("sip"));
    }

    #[test]
    fn display2() {
        assert_eq!(Scheme::Sips.to_string(), String::from("sips"));
    }
}

mod parser {
    use super::*;

    #[test]
    fn parser1() {
        assert_eq!(
            Tokenizer::from("sip".as_bytes()).try_into(),
            Ok(Scheme::Sip)
        );
    }

    #[test]
    fn parser2() {
        assert_eq!(
            Tokenizer::from("sips".as_bytes()).try_into(),
            Ok(Scheme::Sips)
        );
    }
}

mod tokenizer {
    use super::*;

    #[test]
    fn tokenizer1() {
        assert_eq!(
            Tokenizer::tokenize(b"sip:user2@server2.com something"),
            Ok((
                "user2@server2.com something".as_bytes(),
                "sip".as_bytes().into()
            )),
        );
    }

    #[test]
    fn tokenizer2() {
        assert_eq!(
            Tokenizer::tokenize(b"sips:user2@server2.com something"),
            Ok((
                "user2@server2.com something".as_bytes(),
                "sips".as_bytes().into()
            )),
        );
    }

    #[test]
    fn errors1() {
        assert_eq!(
            Tokenizer::tokenize(b"soup:user2@server2.com something"),
            Err(nom::Err::Error(rsip::TokenizerError::from(
                "failed to tokenize scheme: soup:user2@server2.com something"
            ))),
        );
    }

    #[test]
    fn errors2() {
        assert_eq!(
            Tokenizer::tokenize(b"sip//:user2@server2.com something"),
            Err(nom::Err::Error(rsip::TokenizerError::from(
                "failed to tokenize scheme: sip//:user2@server2.com something"
            ))),
        );
    }
}
