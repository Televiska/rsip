use rsip::common::version::{Tokenizer, Version};
use std::convert::{TryFrom, TryInto};

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(Version::V1.to_string(), String::from("SIP/1.0"));
    }

    #[test]
    fn display2() {
        assert_eq!(Version::V2.to_string(), String::from("SIP/2.0"));
    }
}

mod parser {
    use super::*;

    #[test]
    fn parser1() {
        assert_eq!(
            Tokenizer::from(("1".as_bytes(), "0".as_bytes())).try_into(),
            Ok(Version::V1)
        );
    }

    #[test]
    fn parser2() {
        assert_eq!(
            Tokenizer::from(("2".as_bytes(), "0".as_bytes())).try_into(),
            Ok(Version::V2)
        );
    }

    #[test]
    fn errors1() -> Result<(), rsip::Error> {
        assert_eq!(
            Version::try_from(Tokenizer::from(("a".as_bytes(), "0".as_bytes()))),
            Err(rsip::Error::ParseError("Unrecognized SIP version".into()))
        );

        Ok(())
    }
}

mod tokenizer {
    use super::*;

    #[test]
    fn tokenizer1_u8() {
        assert_eq!(
            Tokenizer::tokenize("SIP/1.0\r\nsomething".as_bytes()),
            Ok((
                "\r\nsomething".as_bytes(),
                ("1".as_bytes(), "0".as_bytes()).into()
            )),
        );
    }

    #[test]
    fn tokenizer1_str() {
        assert_eq!(
            Tokenizer::tokenize("SIP/1.0\r\nsomething"),
            Ok(("\r\nsomething", ("1", "0").into())),
        );
    }

    #[test]
    fn tokenizer2_u8() {
        assert_eq!(
            Tokenizer::tokenize("SIP/2.0 something".as_bytes()),
            Ok((
                " something".as_bytes(),
                ("2".as_bytes(), "0".as_bytes()).into()
            )),
        );
    }

    #[test]
    fn tokenizer3_u8() {
        assert_eq!(
            Tokenizer::tokenize("SIP/2.0/UDP pc33.atlanta.com".as_bytes()),
            Ok((
                "/UDP pc33.atlanta.com".as_bytes(),
                ("2".as_bytes(), "0".as_bytes()).into()
            )),
        );
    }

    #[test]
    fn errors1() {
        assert_eq!(
            Tokenizer::tokenize("SIP1.0\r\nsomething".as_bytes()),
            Err(nom::Err::Error(rsip::TokenizerError::from(
                "failed to tokenize version: SIP1.0\r\nsomething"
            ))),
        );
    }
}
