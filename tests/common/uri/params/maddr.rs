use rsip::common::uri::param::maddr::{Maddr, Tokenizer};

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"maddr=255.255.255.255 something"),
        Ok((b" something".as_ref(), b"255.255.255.255".as_ref().into())),
    );

    assert_eq!(
        Tokenizer::tokenize(b"maddr=255.255.255.255;user=filippos"),
        Ok((
            b";user=filippos".as_ref(),
            b"255.255.255.255".as_ref().into()
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b"maddr=255.255.255.255?user=filippos"),
        Ok((
            b"?user=filippos".as_ref(),
            b"255.255.255.255".as_ref().into()
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b"maddr=255.255.255.255\r\nuser=filippos"),
        Ok((
            b"\r\nuser=filippos".as_ref(),
            b"255.255.255.255".as_ref().into()
        )),
    );
}

fn parser() {
    assert_eq!(
        Maddr::parse(b"255.255.255.255".as_ref().into()),
        Ok(Maddr::new("255.255.255.255"))
    );
}
