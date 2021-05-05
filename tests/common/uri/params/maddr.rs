use rsip::common::uri::param::maddr::{Maddr, Tokenizer};

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"maddr=255.255.255.255 something"),
        Ok((" something".as_bytes(), "255.255.255.255".as_bytes().into())),
    );

    assert_eq!(
        Tokenizer::tokenize(b"maddr=255.255.255.255;user=filippos"),
        Ok((
            ";user=filippos".as_bytes(),
            "255.255.255.255".as_bytes().into()
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b"maddr=255.255.255.255?user=filippos"),
        Ok((
            "?user=filippos".as_bytes(),
            "255.255.255.255".as_bytes().into()
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b"maddr=255.255.255.255\r\nuser=filippos"),
        Ok((
            "\r\nuser=filippos".as_bytes(),
            "255.255.255.255".as_bytes().into()
        )),
    );
}

#[test]
fn parser() {
    assert_eq!(
        Maddr::parse("255.255.255.255".as_bytes().into()),
        Ok(Maddr::new("255.255.255.255"))
    );
}
