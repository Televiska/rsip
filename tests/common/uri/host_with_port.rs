use rsip::common::uri::{host_with_port::Tokenizer, Host, HostWithPort};
use std::convert::TryInto;

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(
            HostWithPort {
                host: Host::Domain("server2.com".into()),
                port: None
            }
            .to_string(),
            String::from("server2.com")
        );
    }

    #[test]
    fn display2() {
        assert_eq!(
            HostWithPort {
                host: Host::Domain("server2.com".into()),
                port: Some(5060.into())
            }
            .to_string(),
            String::from("server2.com:5060")
        );
    }
}

mod parser {
    use super::*;

    #[test]
    fn parser1() {
        assert_eq!(
            Tokenizer::from(("server2.com".as_bytes(), None)).try_into(),
            Ok(HostWithPort {
                host: Host::Domain("server2.com".into()),
                port: None
            })
        );
    }

    #[test]
    fn parser2() {
        assert_eq!(
            Tokenizer::from(("server2.com".as_bytes(), Some("5060".as_bytes()))).try_into(),
            Ok(HostWithPort {
                host: Host::Domain("server2.com".into()),
                port: Some(5060.into())
            })
        );
    }
}

mod tokenizer {
    use super::*;

    #[test]
    fn tokenizer1_u8() {
        assert_eq!(
            Tokenizer::tokenize("server2.com something".as_bytes()),
            Ok((
                " something".as_bytes(),
                ("server2.com".as_bytes(), None).into()
            )),
        );
    }

    #[test]
    fn tokenizer1_str() {
        assert_eq!(
            Tokenizer::tokenize("server2.com something"),
            Ok((" something", ("server2.com", None).into())),
        );
    }

    #[test]
    fn tokenizer2_u8() {
        assert_eq!(
            Tokenizer::tokenize("server2.com:5060 something".as_bytes()),
            Ok((
                " something".as_bytes(),
                ("server2.com".as_bytes(), Some("5060".as_bytes())).into()
            )),
        );
    }

    #[test]
    fn errors1() {
        assert_eq!(
            Tokenizer::tokenize(";".as_bytes()),
            Err(nom::Err::Error(rsip::TokenizerError::from(
                "failed to tokenize host with port: ;"
            ))),
        );
    }
}
