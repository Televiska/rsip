use rsip::common::uri::schema::{Schema, Tokenizer};

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"sip:user2@server2.com SIP/2.0"),
        Ok((
            b"user2@server2.com SIP/2.0".as_ref(),
            b"sip".as_ref().into()
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b"sips:user2@server2.com SIP/2.0"),
        Ok((
            b"user2@server2.com SIP/2.0".as_ref(),
            b"sips".as_ref().into()
        )),
    );
}

#[test]
fn parser() {
    assert_eq!(Schema::parse(b"sip".as_ref().into()), Ok(Schema::Sip));
    assert_eq!(Schema::parse(b"sips".as_ref().into()), Ok(Schema::Sips));
}
