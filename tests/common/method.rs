use rsip::common::method::{Method, Tokenizer};

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"REGISTER something"),
        Ok((b"something".as_ref(), b"REGISTER".as_ref().into())),
    );
}

#[test]
fn parser() {
    assert_eq!(
        Method::parse(b"REGISTER".as_ref().into()),
        Ok(Method::Register),
    );
}
