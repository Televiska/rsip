use rsip::{
    headers::auth::{AuthQop, Scheme},
    headers::typed::{authorization::Tokenizer, Authorization},
};
use std::convert::TryInto;

validate_typed_header_trait!(Authorization);

#[test]
fn display() -> Result<(), rsip::Error> {
    use rsip::headers::auth;

    assert_eq!(
        format!(
            "{}",
            Authorization {
                scheme: auth::Scheme::Digest,
                realm: "http-auth@example.org".into(),
                username: "Mufasa".into(),
                uri: "/dir/index.html".try_into()?,
                algorithm: Some(auth::Algorithm::Md5),
                nonce: "7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v".into(),
                qop: Some(auth::AuthQop::Auth {
                    cnonce: "f2/wE4q74E6zIJEtWaHKaf5wv/H5QzzpXusqGemxURZJ".into(),
                    nc: 1
                }),
                response: "8ca523f5e9506fed4657c9700eebdbec".into(),
                opaque: Some("FQhe/qaU925kfnzjCev0ciny7QMkPqMAFRtzCUYo5tdS".into())
            }
        ),
        String::from(concat!(
            "Digest ",
            "username=\"Mufasa\", ",
            "realm=\"http-auth@example.org\", ",
            "nonce=\"7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v\", ",
            "uri=\"/dir/index.html\", ",
            "response=\"8ca523f5e9506fed4657c9700eebdbec\", ",
            "algorithm=MD5, ",
            "opaque=\"FQhe/qaU925kfnzjCev0ciny7QMkPqMAFRtzCUYo5tdS\", ",
            "qop=\"auth\", ",
            "nc=00000001, ",
            "cnonce=\"f2/wE4q74E6zIJEtWaHKaf5wv/H5QzzpXusqGemxURZJ\""
        ))
    );

    Ok(())
}

#[test]
fn from_tokenizer() {
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
        Ok(Authorization {
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
        Ok(Authorization {
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
