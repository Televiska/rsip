use rsip::headers::header::www_authenticate::{self, Tokenizer};
use rsip::headers::header::Tokenize;

validate_untyped_header_trait!(www_authenticate, WwwAuthenticate);

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(concat!(
            "Digest ",
            "realm=\"testrealm@host.com\", ",
            "qop=\"auth,auth-int\", ",
            "nonce=\"dcd98b7102dd2f0e8b11d0f600bfb0c093\", ",
            "opaque=\"5ccc069c403ebaf9f0171e9517f40e41\", "
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

    //with space in-between in qop values
    assert_eq!(
        Tokenizer::tokenize(concat!(
            "Digest ",
            "realm=\"testrealm@host.com\", ",
            "qop=\"auth, auth-int\", ",
            "nonce=\"dcd98b7102dd2f0e8b11d0f600bfb0c093\", ",
            "opaque=\"5ccc069c403ebaf9f0171e9517f40e41\", "
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

mod typed {
    use super::*;
    use rsip::common::auth;
    use std::convert::TryInto;

    validate_typed_header_trait!(www_authenticate, WwwAuthenticate);

    #[test]
    fn typed() {
        assert_eq!(
            Tokenizer {
                scheme: "Digest".into(),
                params: vec![
                    ("realm", "testrealm@host.com"),
                    ("qop", "auth,auth-int"),
                    ("nonce", "dcd98b7102dd2f0e8b11d0f600bfb0c093"),
                    ("opaque", "5ccc069c403ebaf9f0171e9517f40e41"),
                ],
            }
            .try_into(),
            Ok(www_authenticate::typed::WwwAuthenticate {
                scheme: auth::Scheme::Digest,
                realm: "testrealm@host.com".into(),
                domain: None,
                nonce: "dcd98b7102dd2f0e8b11d0f600bfb0c093".into(),
                opaque: Some("5ccc069c403ebaf9f0171e9517f40e41".into()),
                stale: None,
                algorithm: None,
                qop: Some("auth,auth-int".into()),
                charset: None
            })
        );
    }
}
