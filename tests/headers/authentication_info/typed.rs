use rsip::{
    headers::auth::AuthQop,
    headers::typed::{authentication_info::Tokenizer, AuthenticationInfo},
};
use std::convert::TryInto;

validate_typed_header_trait!(AuthenticationInfo);

mod display {
    use super::*;

    #[test]
    fn display1() -> Result<(), rsip::Error> {
        use rsip::headers::auth;

        assert_eq!(
            format!(
                "{}",
                AuthenticationInfo {
                    nextnonce: "7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v".into(),
                    qop: Some(auth::AuthQop::Auth {
                        cnonce: "f2/wE4q74E6zIJEtWaHKaf5wv/H5QzzpXusqGemxURZJ".into(),
                        nc: 1
                    }),
                    rspauth: Some("8ca523f5e9506fed4657c9700eebdbec".into()),
                }
            ),
            String::from(concat!(
                "nextnonce=\"7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v\", ",
                "rspauth=\"8ca523f5e9506fed4657c9700eebdbec\", ",
                "qop=\"auth\", ",
                "nc=00000001, ",
                "cnonce=\"f2/wE4q74E6zIJEtWaHKaf5wv/H5QzzpXusqGemxURZJ\""
            ))
        );

        Ok(())
    }

    #[test]
    fn display2() -> Result<(), rsip::Error> {
        assert_eq!(
            format!(
                "{}",
                AuthenticationInfo {
                    nextnonce: "7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v".into(),
                    qop: None,
                    rspauth: Some("8ca523f5e9506fed4657c9700eebdbec".into()),
                }
            ),
            String::from(concat!(
                "nextnonce=\"7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v\", ",
                "rspauth=\"8ca523f5e9506fed4657c9700eebdbec\"",
            ))
        );

        Ok(())
    }
}

mod try_from_tokenizer {
    use super::*;

    #[test]
    fn try_from1() {
        assert_eq!(
            Tokenizer {
                params: vec![
                    ("nextnonce", "ea9c8e88df84f1cec4341ae6cbe5a359"),
                    ("rspauth", "dfe56131d1958046689d83306477ecc"),
                ],
            }
            .try_into(),
            Ok(AuthenticationInfo {
                nextnonce: "ea9c8e88df84f1cec4341ae6cbe5a359".into(),
                rspauth: Some("dfe56131d1958046689d83306477ecc".into()),
                qop: None
            })
        );
    }

    #[test]
    fn try_from2() {
        assert_eq!(
            Tokenizer {
                params: vec![
                    ("nextnonce", "ea9c8e88df84f1cec4341ae6cbe5a359"),
                    ("rspauth", "dfe56131d1958046689d83306477ecc"),
                    ("qop", "auth"),
                    ("cnonce", "0a4f113b"),
                    ("nc", "00000001")
                ],
            }
            .try_into(),
            Ok(AuthenticationInfo {
                nextnonce: "ea9c8e88df84f1cec4341ae6cbe5a359".into(),
                rspauth: Some("dfe56131d1958046689d83306477ecc".into()),
                qop: Some(AuthQop::Auth {
                    cnonce: "0a4f113b".into(),
                    nc: 1
                })
            })
        );
    }
}
