use rsip::common::version::{Tokenizer, Version};
use std::convert::TryInto;

#[test]
fn display() {
    assert_eq!(Version::V1.to_string(), String::from("SIP/1.0"));

    assert_eq!(Version::V2.to_string(), String::from("SIP/2.0"));
}

#[test]
fn parser() {
    assert_eq!(Tokenizer::from("1".as_bytes()).try_into(), Ok(Version::V1));

    assert_eq!(Tokenizer::from("2".as_bytes()).try_into(), Ok(Version::V2));
}

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"SIP/1.0\r\nsomething"),
        Ok(("\r\nsomething".as_bytes(), "1".as_bytes().into())),
    );

    assert_eq!(
        Tokenizer::tokenize(b"SIP/2.0 something"),
        Ok((" something".as_bytes(), "2".as_bytes().into())),
    );

    assert_eq!(
        Tokenizer::tokenize(b"SIP/2.0/UDP pc33.atlanta.com"),
        Ok(("/UDP pc33.atlanta.com".as_bytes(), "2".as_bytes().into())),
    );
}
