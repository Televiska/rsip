use rsip::common::uri::schema::{Schema, Tokenizer};
use std::convert::TryInto;

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"sip:user2@server2.com something"),
        Ok((
            "user2@server2.com something".as_bytes(),
            "sip".as_bytes().into()
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b"sips:user2@server2.com something"),
        Ok((
            "user2@server2.com something".as_bytes(),
            "sips".as_bytes().into()
        )),
    );
}

#[test]
fn parser() {
    assert_eq!(
        Tokenizer::from("sip".as_bytes()).try_into(),
        Ok(Schema::Sip)
    );
    assert_eq!(
        Tokenizer::from("sips".as_bytes()).try_into(),
        Ok(Schema::Sips)
    );
}
