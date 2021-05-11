use rsip::common::uri::param::Tokenizer;

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b";maddr=255.255.255.255;something"),
        Ok((
            ";something".as_bytes(),
            ("maddr".as_bytes(), Some("255.255.255.255".as_bytes())).into()
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b";maddr=255.255.255.255;something"),
        Ok((
            ";something".as_bytes(),
            ("maddr".as_bytes(), Some("255.255.255.255".as_bytes())).into()
        )),
    );
}
