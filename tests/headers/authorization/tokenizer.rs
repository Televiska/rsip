use rsip::headers::typed::{authorization::Tokenizer, Tokenize};

#[test]
fn tokenizer1() {
    assert_eq!(
        Tokenizer::tokenize(concat!(
            "Digest ",
            "username=\"bob\", ",
            "realm=\"atlanta.example.com\", ",
            "nonce=\"ea9c8e88df84f1cec4341ae6cbe5a359\", ",
            "opaque=\"\", ",
            "uri=\"sips:ss2.biloxi.example.com\", ",
            "response=\"dfe56131d1958046689d83306477ecc\""
        )),
        Ok(Tokenizer {
            scheme: "Digest".into(),
            params: vec![
                ("username", "bob"),
                ("realm", "atlanta.example.com"),
                ("nonce", "ea9c8e88df84f1cec4341ae6cbe5a359"),
                ("opaque", ""),
                ("uri", "sips:ss2.biloxi.example.com"),
                ("response", "dfe56131d1958046689d83306477ecc")
            ],
        })
    );
}
