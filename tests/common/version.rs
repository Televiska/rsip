use rsip::common::version::{Tokenizer, Version};

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"SIP/1.0 something"),
        Ok((b"something".as_ref(), b"SIP/1.0".as_ref().into())),
    );
}

#[test]
fn parser() {
    assert_eq!(Version::parse(b"SIP/1.0".as_ref().into()), Ok(Version::V1));

    assert_eq!(Version::parse(b"SIP/2.0".as_ref().into()), Ok(Version::V2));
}
