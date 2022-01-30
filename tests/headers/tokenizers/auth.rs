use rsip::headers::typed::{tokenizers::AuthTokenizer, Tokenize};

#[test]
fn tokenizer1() {
    assert_eq!(
        AuthTokenizer::tokenize(concat!(
            "Digest ",
            "realm=\"testrealm@host.com\", ",
            "qop=\"auth,auth-int\", ",
            "nonce=\"dcd98b7102dd2f0e8b11d0f600bfb0c093\", ",
            "opaque=\"5ccc069c403ebaf9f0171e9517f40e41\""
        )),
        Ok(AuthTokenizer {
            scheme: "Digest".into(),
            params: vec![
                ("realm", "testrealm@host.com"),
                ("qop", "auth,auth-int"),
                ("nonce", "dcd98b7102dd2f0e8b11d0f600bfb0c093"),
                ("opaque", "5ccc069c403ebaf9f0171e9517f40e41"),
            ],
        })
    );
}

#[test]
fn tokenizer2() {
    //with space in-between in qop values
    assert_eq!(
        AuthTokenizer::tokenize(concat!(
            "Digest ",
            "realm=\"testrealm@host.com\", ",
            "qop=\"auth, auth-int\", ",
            "nonce=\"dcd98b7102dd2f0e8b11d0f600bfb0c093\", ",
            "opaque=\"5ccc069c403ebaf9f0171e9517f40e41\""
        )),
        Ok(AuthTokenizer {
            scheme: "Digest".into(),
            params: vec![
                ("realm", "testrealm@host.com"),
                ("qop", "auth, auth-int"),
                ("nonce", "dcd98b7102dd2f0e8b11d0f600bfb0c093"),
                ("opaque", "5ccc069c403ebaf9f0171e9517f40e41"),
            ],
        })
    );
}

#[test]
fn tokenizer3() {
    assert_eq!(
        AuthTokenizer::tokenize(concat!(
            "Digest ",
            "username=\"bob\", ",
            "realm=\"atlanta.example.com\", ",
            "nonce=\"ea9c8e88df84f1cec4341ae6cbe5a359\", ",
            "opaque=\"\", ",
            "uri=\"sips:ss2.biloxi.example.com\", ",
            "response=\"dfe56131d1958046689d83306477ecc\""
        )),
        Ok(AuthTokenizer {
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

#[test]
fn tokenizer4() {
    assert_eq!(
        AuthTokenizer::tokenize(concat!(
            "Digest ",
            "username=\"bob\", ",
            "realm=\"atlanta.example.com\", ",
            "nonce=\"ea9c8e88df84f1cec4341ae6cbe5a359\", ",
            "opaque=\"\", ",
            "uri=\"sips:ss2.biloxi.example.com\", ",
            "response=dfe56131d1958046689d83306477ecc"
        )),
        Ok(AuthTokenizer {
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
