use rsip::common::transport::{Tokenizer, Transport};
use std::convert::TryInto;

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(Transport::Udp.to_string(), String::from("UDP"));

        assert_eq!(Transport::Tcp.to_string(), String::from("TCP"));
    }
}

mod parser {
    use super::*;

    #[test]
    fn parser1() {
        assert_eq!(
            Tokenizer::from("UDP".as_bytes()).try_into(),
            Ok(Transport::Udp)
        );
    }

    #[test]
    fn parser2() {
        assert_eq!(
            Tokenizer::from("TCP".as_bytes()).try_into(),
            Ok(Transport::Tcp)
        );
    }
}

mod tokenizer {
    use super::*;

    #[test]
    fn tokenizer1() {
        assert_eq!(
            Tokenizer::tokenize(b"UDP "),
            Ok((" ".as_bytes(), "UDP".as_bytes().into())),
        );
    }

    #[test]
    fn tokenizer2() {
        assert_eq!(
            Tokenizer::tokenize(b"TCP"),
            Ok(("".as_bytes(), "TCP".as_bytes().into())),
        );
    }
}
