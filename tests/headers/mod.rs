pub mod accept;

use rsip::headers::header::Tokenizer;

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"Accept: REGISTER, INVITE\r\n something"),
        Ok((
            " something".as_bytes(),
            Tokenizer {
                name: "Accept".as_bytes(),
                value: "REGISTER, INVITE".as_bytes()
            }
        )),
    );
}

/*
#[test]
fn parser() {
    assert_eq!(
        Accept::parse(b"REGISTER, INVITE".as_bytes().into()),
        Ok(Accept::new("REGISTER, INVITE")),
    );
}*/
