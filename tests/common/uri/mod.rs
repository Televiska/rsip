pub mod auth;
pub mod host_with_port;
pub mod params;
pub mod schema;

use rsip::common::uri::{Schema, Tokenizer, Uri};

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"server2.com SIP/2.0"),
        Ok((
            "SIP/2.0".as_bytes(),
            Tokenizer {
                schema: None,
                auth: None,
                host_with_port: ("server2.com".as_bytes(), None).into(),
                params: None,
                headers: None
            }
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b"user@server2.com SIP/2.0"),
        Ok((
            "SIP/2.0".as_bytes(),
            Tokenizer {
                schema: None,
                auth: Some(("user".as_bytes(), None).into()),
                host_with_port: ("server2.com".as_bytes(), None).into(),
                params: None,
                headers: None
            }
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b"user:password@server2.com SIP/2.0"),
        Ok((
            "SIP/2.0".as_bytes(),
            Tokenizer {
                schema: None,
                auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
                host_with_port: ("server2.com".as_bytes(), None).into(),
                params: None,
                headers: None
            }
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b"user:password@server2.com:5060 SIP/2.0"),
        Ok((
            "SIP/2.0".as_bytes(),
            Tokenizer {
                schema: None,
                auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
                host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
                params: None,
                headers: None
            }
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b"sip:user@server2.com:5060 SIP/2.0"),
        Ok((
            "SIP/2.0".as_bytes(),
            Tokenizer {
                schema: Some("sip".as_bytes().into()),
                auth: Some(("user".as_bytes(), None).into()),
                host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
                params: None,
                headers: None
            }
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b"sip:user:password@server2.com:5060 SIP/2.0"),
        Ok((
            "SIP/2.0".as_bytes(),
            Tokenizer {
                schema: Some("sip".as_bytes().into()),
                auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
                host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
                params: None,
                headers: None
            }
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b"sips:ss2.biloxi.example.com SIP/2.0"),
        Ok((
            "SIP/2.0".as_bytes(),
            Tokenizer {
                schema: Some("sips".as_bytes().into()),
                auth: None,
                host_with_port: ("ss2.biloxi.example.com".as_bytes(), None).into(),
                params: None,
                headers: None
            }
        )),
    );
}

#[test]
fn parser() {
    assert_eq!(
        Uri::parse(Tokenizer {
            schema: None,
            auth: None,
            host_with_port: ("server2.com".as_bytes(), None).into(),
            params: None,
            headers: None
        }),
        Ok(Uri {
            schema: None,
            auth: None,
            host_with_port: ("server2.com", Option::<u16>::None).into(),
            params: Default::default(),
            headers: Default::default()
        })
    );

    assert_eq!(
        Uri::parse(Tokenizer {
            schema: None,
            auth: Some(("user".as_bytes(), None).into()),
            host_with_port: ("server2.com".as_bytes(), None).into(),
            params: None,
            headers: None
        }),
        Ok(Uri {
            schema: None,
            auth: Some(("user", Option::<String>::None).into()),
            host_with_port: ("server2.com", Option::<u16>::None).into(),
            params: Default::default(),
            headers: Default::default()
        })
    );

    assert_eq!(
        Uri::parse(Tokenizer {
            schema: None,
            auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
            host_with_port: ("server2.com".as_bytes(), None).into(),
            params: None,
            headers: None
        }),
        Ok(Uri {
            schema: None,
            auth: Some(("user", Some("password")).into()),
            host_with_port: ("server2.com", Option::<u16>::None).into(),
            params: Default::default(),
            headers: Default::default()
        })
    );

    assert_eq!(
        Uri::parse(Tokenizer {
            schema: None,
            auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
            host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
            params: None,
            headers: None
        }),
        Ok(Uri {
            schema: None,
            auth: Some(("user", Some("password")).into()),
            host_with_port: ("server2.com", Some(5060)).into(),
            params: Default::default(),
            headers: Default::default()
        })
    );

    assert_eq!(
        Uri::parse(Tokenizer {
            schema: Some("sip".as_bytes().into()),
            auth: Some(("user".as_bytes(), None).into()),
            host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
            params: None,
            headers: None
        }),
        Ok(Uri {
            schema: Some(Schema::Sip),
            auth: Some(("user", Option::<String>::None).into()),
            host_with_port: ("server2.com", Some(5060)).into(),
            params: Default::default(),
            headers: Default::default()
        })
    );

    assert_eq!(
        Uri::parse(Tokenizer {
            schema: Some("sip".as_bytes().into()),
            auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
            host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
            params: None,
            headers: None
        }),
        Ok(Uri {
            schema: Some(Schema::Sip),
            auth: Some(("user", Some("password")).into()),
            host_with_port: ("server2.com", Some(5060)).into(),
            params: Default::default(),
            headers: Default::default()
        })
    );
}
