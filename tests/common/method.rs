use rsip::common::method::{Method, Tokenizer};

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"REGISTER something"),
        Ok(("something".as_bytes(), "REGISTER".as_bytes().into())),
    );
}

#[test]
fn parser() {
    assert_eq!(
        Method::parse("REGISTER".as_bytes().into()),
        Ok(Method::Register),
    );
}
