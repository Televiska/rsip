pub mod accept;

use rsip::headers::header::Tokenizer;

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"Accept: REGISTER, INVITE\r\n something"),
        Ok((
            b" something".as_ref(),
            Tokenizer {
                name: b"Accept".as_ref(),
                value: b"REGISTER, INVITE".as_ref()
            }
        )),
    );
}

/*
#[test]
fn parser() {
    assert_eq!(
        Accept::parse(b"REGISTER, INVITE".as_ref().into()),
        Ok(Accept::new("REGISTER, INVITE")),
    );
}*/
