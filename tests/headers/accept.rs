use rsip::headers::accept::{Accept, Tokenizer};

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"Accept: REGISTER, INVITE\r\n something"),
        Ok((
            b" something".as_ref(),
            Tokenizer {
                value: b"REGISTER, INVITE".as_ref()
            }
        )),
    );
}

#[test]
fn parser() {
    assert_eq!(
        Accept::parse(b"REGISTER, INVITE".as_ref().into()),
        Ok(Accept::new("REGISTER, INVITE")),
    );
}

#[test]
fn value() {
    let header = Accept::new("A value");
    assert_eq!(header.value(), "A value");
}

#[test]
fn display() {
    let header = Accept::new("A value");
    assert_eq!(&format!("{}", header), "Accept: A value");
}
