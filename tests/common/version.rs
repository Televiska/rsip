use rsip::common::version::{Tokenizer, Version};
use std::convert::TryInto;

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"SIP/1.0\r\nsomething"),
        Ok(("something".as_bytes(), "SIP/1.0".as_bytes().into())),
    );

    assert_eq!(
        Tokenizer::tokenize(b"SIP/1.0 something"),
        Ok(("something".as_bytes(), "SIP/1.0".as_bytes().into())),
    );
}

#[test]
fn parser() {
    assert_eq!(
        Tokenizer::from("SIP/1.0".as_bytes()).try_into(),
        Ok(Version::V1)
    );

    assert_eq!(
        Tokenizer::from("SIP/2.0".as_bytes()).try_into(),
        Ok(Version::V2)
    );
}
