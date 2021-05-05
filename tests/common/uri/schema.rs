use rsip::common::uri::schema::{Schema, Tokenizer};

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"sip:user2@server2.com SIP/2.0"),
        Ok((
            "user2@server2.com SIP/2.0".as_bytes(),
            "sip".as_bytes().into()
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b"sips:user2@server2.com SIP/2.0"),
        Ok((
            "user2@server2.com SIP/2.0".as_bytes(),
            "sips".as_bytes().into()
        )),
    );
}

#[test]
fn parser() {
    assert_eq!(Schema::parse("sip".as_bytes().into()), Ok(Schema::Sip));
    assert_eq!(Schema::parse("sips".as_bytes().into()), Ok(Schema::Sips));
}
