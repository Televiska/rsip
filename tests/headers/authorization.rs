use rsip::headers::header::authorization::{self, Tokenizer};
use rsip::headers::header::Tokenize;

validate_untyped_header_trait!(authorization, Authorization);

#[test]
fn tokenizer() {
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

mod typed {
    use super::*;
    use rsip::common::auth::{AuthQop, Scheme};
    use std::convert::TryInto;

    validate_typed_header_trait!(authorization, Authorization);

    #[test]
    fn typed() {
        assert_eq!(
            Tokenizer {
                scheme: "Digest".into(),
                params: vec![
                    ("username", "bob"),
                    ("realm", "atlanta.example.com"),
                    ("nonce", "ea9c8e88df84f1cec4341ae6cbe5a359"),
                    ("opaque", ""),
                    ("uri", "sips:ss2.biloxi.example.com"),
                    ("response", "dfe56131d1958046689d83306477ecc")
                ],
            }
            .try_into(),
            Ok(authorization::typed::Authorization {
                scheme: Scheme::Digest,
                username: "bob".into(),
                realm: "atlanta.example.com".into(),
                nonce: "ea9c8e88df84f1cec4341ae6cbe5a359".into(),
                uri: "sips:ss2.biloxi.example.com"
                    .try_into()
                    .expect("parsing uri"),
                response: "dfe56131d1958046689d83306477ecc".into(),
                opaque: Some("".into()),
                algorithm: None,
                qop: None
            })
        );

        assert_eq!(
            Tokenizer {
                scheme: "Digest".into(),
                params: vec![
                    ("username", "bob"),
                    ("realm", "atlanta.example.com"),
                    ("nonce", "ea9c8e88df84f1cec4341ae6cbe5a359"),
                    ("opaque", ""),
                    ("uri", "sips:ss2.biloxi.example.com"),
                    ("response", "dfe56131d1958046689d83306477ecc"),
                    ("qop", "auth"),
                    ("cnonce", "0a4f113b"),
                    ("nc", "00000001")
                ],
            }
            .try_into(),
            Ok(authorization::typed::Authorization {
                scheme: Scheme::Digest,
                username: "bob".into(),
                realm: "atlanta.example.com".into(),
                nonce: "ea9c8e88df84f1cec4341ae6cbe5a359".into(),
                uri: "sips:ss2.biloxi.example.com"
                    .try_into()
                    .expect("parsing uri"),
                response: "dfe56131d1958046689d83306477ecc".into(),
                opaque: Some("".into()),
                algorithm: None,
                qop: Some(AuthQop::Auth {
                    cnonce: "0a4f113b".into(),
                    nc: 1
                })
            })
        );
    }
}
