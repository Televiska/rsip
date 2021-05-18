use rsip::common::uri::param::{Maddr, Param, Tokenizer};
use std::convert::TryInto;

#[test]
fn display() {
    assert_eq!(
        Param::Maddr(Maddr::new("255.255.255.0")).to_string(),
        String::from(";maddr=255.255.255.0")
    );
}

#[test]
fn parser() {
    assert_eq!(
        Tokenizer {
            name: "maddr".as_bytes(),
            value: Some("255.255.255.0".as_bytes()),
        }
        .try_into(),
        Ok(Param::Maddr(Maddr::new("255.255.255.0")))
    );

    assert_eq!(
        Tokenizer {
            name: "maddr".as_bytes(),
            value: None,
        }
        .try_into(),
        Ok(Param::Other("maddr".into(), None))
    );
}

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b";maddr=255.255.255.255;something"),
        Ok((
            ";something".as_bytes(),
            ("maddr".as_bytes(), Some("255.255.255.255".as_bytes())).into()
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b";maddr=255.255.255.255;something"),
        Ok((
            ";something".as_bytes(),
            ("maddr".as_bytes(), Some("255.255.255.255".as_bytes())).into()
        )),
    );
}
