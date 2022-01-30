use rsip::headers::typed::{tokenizers::NameValueTokenizer, Tokenize};

#[test]
fn tokenizer1() {
    assert_eq!(
        NameValueTokenizer::tokenize(concat!(
            "realm=\"testrealm@host.com\", ",
            "qop=\"auth,auth-int\", ",
            "nonce=\"dcd98b7102dd2f0e8b11d0f600bfb0c093\", ",
            "opaque=\"5ccc069c403ebaf9f0171e9517f40e41\""
        )),
        Ok(NameValueTokenizer {
            params: vec![
                ("realm", "testrealm@host.com"),
                ("qop", "auth,auth-int"),
                ("nonce", "dcd98b7102dd2f0e8b11d0f600bfb0c093"),
                ("opaque", "5ccc069c403ebaf9f0171e9517f40e41"),
            ]
        })
    );
}

#[test]
fn tokenizer2() {
    assert_eq!(
        NameValueTokenizer::tokenize(concat!(
            "foo=1, ",
            "with_quotes=\"1\", ",
            "without_space=\"1\",",
            "bar=foobar, ",
            "response=dfe56131d1958046689d83306477ecc"
        )),
        Ok(NameValueTokenizer {
            params: vec![
                ("foo", "1"),
                ("with_quotes", "1"),
                ("without_space", "1"),
                ("bar", "foobar"),
                ("response", "dfe56131d1958046689d83306477ecc"),
            ]
        })
    );
}
