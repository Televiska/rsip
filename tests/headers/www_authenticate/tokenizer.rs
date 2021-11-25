use rsip::headers::typed::{www_authenticate::Tokenizer, Tokenize};

#[test]
fn tokenizer1() {
    assert_eq!(
        Tokenizer::tokenize(concat!(
            "Digest ",
            "realm=\"testrealm@host.com\", ",
            "qop=\"auth,auth-int\", ",
            "nonce=\"dcd98b7102dd2f0e8b11d0f600bfb0c093\", ",
            "opaque=\"5ccc069c403ebaf9f0171e9517f40e41\""
        )),
        Ok(Tokenizer {
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
        Tokenizer::tokenize(concat!(
            "Digest ",
            "realm=\"testrealm@host.com\", ",
            "qop=\"auth, auth-int\", ",
            "nonce=\"dcd98b7102dd2f0e8b11d0f600bfb0c093\", ",
            "opaque=\"5ccc069c403ebaf9f0171e9517f40e41\""
        )),
        Ok(Tokenizer {
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
