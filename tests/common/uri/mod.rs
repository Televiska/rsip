pub mod auth;
pub mod host_with_port;
pub mod params;
pub mod scheme;

use rsip::common::uri::{param::Maddr, Param, Scheme, Tokenizer, Uri};
use std::convert::TryInto;

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(
            Uri {
                scheme: None,
                auth: None,
                host_with_port: ("server2.com", Option::<u16>::None).into(),
                params: Default::default(),
                headers: Default::default()
            }
            .to_string(),
            String::from("server2.com")
        );
    }

    #[test]
    fn display2() {
        assert_eq!(
            Uri {
                scheme: None,
                auth: Some(("user", Option::<String>::None).into()),
                host_with_port: ("server2.com", Option::<u16>::None).into(),
                params: Default::default(),
                headers: Default::default()
            }
            .to_string(),
            String::from("user@server2.com")
        );
    }

    #[test]
    fn display3() {
        assert_eq!(
            Uri {
                scheme: None,
                auth: Some(("user", Some("password")).into()),
                host_with_port: ("server2.com", Option::<u16>::None).into(),
                params: Default::default(),
                headers: Default::default()
            }
            .to_string(),
            String::from("user:password@server2.com")
        );
    }

    #[test]
    fn display4() {
        assert_eq!(
            Uri {
                scheme: None,
                auth: Some(("user", Some("password")).into()),
                host_with_port: ("server2.com", Some(5060)).into(),
                params: Default::default(),
                headers: Default::default()
            }
            .to_string(),
            String::from("user:password@server2.com:5060")
        );
    }

    #[test]
    fn display5() {
        assert_eq!(
            Uri {
                scheme: Some(Scheme::Sips),
                auth: None,
                host_with_port: ("client.biloxi.example.com", Some(5061)).into(),
                params: vec![
                    Param::Maddr(Maddr::new("255.255.255.0")),
                    Param::Other("foo".into(), Some("192.0.2.201".into())),
                ],
                headers: Default::default()
            }
            .to_string(),
            String::from("sips:client.biloxi.example.com:5061;maddr=255.255.255.0;foo=192.0.2.201")
        );
    }
}

mod parser {
    use super::*;

    #[test]
    fn parser1() {
        assert_eq!(
            Tokenizer {
                scheme: None,
                auth: None,
                host_with_port: ("server2.com".as_bytes(), None).into(),
                params: vec![],
                headers: None
            }
            .try_into(),
            Ok(Uri {
                scheme: None,
                auth: None,
                host_with_port: ("server2.com", Option::<u16>::None).into(),
                params: Default::default(),
                headers: Default::default()
            })
        );
    }

    #[test]
    fn parser2() {
        assert_eq!(
            Tokenizer {
                scheme: None,
                auth: Some(("user".as_bytes(), None).into()),
                host_with_port: ("server2.com".as_bytes(), None).into(),
                params: vec![],
                headers: None
            }
            .try_into(),
            Ok(Uri {
                scheme: None,
                auth: Some(("user", Option::<String>::None).into()),
                host_with_port: ("server2.com", Option::<u16>::None).into(),
                params: Default::default(),
                headers: Default::default()
            })
        );
    }

    #[test]
    fn parser3() {
        assert_eq!(
            Tokenizer {
                scheme: None,
                auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
                host_with_port: ("server2.com".as_bytes(), None).into(),
                params: vec![],
                headers: None
            }
            .try_into(),
            Ok(Uri {
                scheme: None,
                auth: Some(("user", Some("password")).into()),
                host_with_port: ("server2.com", Option::<u16>::None).into(),
                params: Default::default(),
                headers: Default::default()
            })
        );
    }

    #[test]
    fn parser4() {
        assert_eq!(
            Tokenizer {
                scheme: None,
                auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
                host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
                params: vec![],
                headers: None
            }
            .try_into(),
            Ok(Uri {
                scheme: None,
                auth: Some(("user", Some("password")).into()),
                host_with_port: ("server2.com", Some(5060)).into(),
                params: Default::default(),
                headers: Default::default()
            })
        );
    }

    #[test]
    fn parser5() {
        assert_eq!(
            Tokenizer {
                scheme: Some("sip".as_bytes().into()),
                auth: Some(("user".as_bytes(), None).into()),
                host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
                params: vec![],
                headers: None
            }
            .try_into(),
            Ok(Uri {
                scheme: Some(Scheme::Sip),
                auth: Some(("user", Option::<String>::None).into()),
                host_with_port: ("server2.com", Some(5060)).into(),
                params: Default::default(),
                headers: Default::default()
            })
        );
    }

    #[test]
    fn parser6() {
        assert_eq!(
            Tokenizer {
                scheme: Some("sip".as_bytes().into()),
                auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
                host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
                params: vec![],
                headers: None
            }
            .try_into(),
            Ok(Uri {
                scheme: Some(Scheme::Sip),
                auth: Some(("user", Some("password")).into()),
                host_with_port: ("server2.com", Some(5060)).into(),
                params: Default::default(),
                headers: Default::default()
            })
        );
    }

    #[test]
    fn parser7() {
        assert_eq!(
            Tokenizer {
                scheme: Some("sips".as_bytes().into()),
                auth: None,
                host_with_port: (
                    "client.biloxi.example.com".as_bytes(),
                    Some("5061".as_bytes())
                )
                    .into(),
                params: vec![
                    ("maddr".as_bytes(), Some("255.255.255.0".as_bytes())).into(),
                    ("foo".as_bytes(), Some("192.0.2.201".as_bytes())).into()
                ],
                headers: None
            }
            .try_into(),
            Ok(Uri {
                scheme: Some(Scheme::Sips),
                auth: None,
                host_with_port: ("client.biloxi.example.com", Some(5061)).into(),
                params: vec![
                    Param::Maddr(Maddr::new("255.255.255.0")),
                    Param::Other("foo".into(), Some("192.0.2.201".into())),
                ],
                headers: Default::default()
            })
        );
    }
}

mod tokenizer {
    use super::*;

    #[test]
    fn tokenizer1() {
        assert_eq!(
            Tokenizer::tokenize(b"server2.com something"),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: None,
                    auth: None,
                    host_with_port: ("server2.com".as_bytes(), None).into(),
                    params: vec![],
                    headers: None
                }
            )),
        );
    }

    #[test]
    fn tokenizer2() {
        assert_eq!(
            Tokenizer::tokenize(b"user@server2.com something"),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: None,
                    auth: Some(("user".as_bytes(), None).into()),
                    host_with_port: ("server2.com".as_bytes(), None).into(),
                    params: vec![],
                    headers: None
                }
            )),
        );
    }

    #[test]
    fn tokenizer3() {
        assert_eq!(
            Tokenizer::tokenize(b"user:password@server2.com something"),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: None,
                    auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
                    host_with_port: ("server2.com".as_bytes(), None).into(),
                    params: vec![],
                    headers: None
                }
            )),
        );
    }

    #[test]
    fn tokenizer4() {
        assert_eq!(
            Tokenizer::tokenize(b"user:password@server2.com:5060 something"),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: None,
                    auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
                    host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
                    params: vec![],
                    headers: None
                }
            )),
        );
    }

    #[test]
    fn tokenizer5() {
        assert_eq!(
            Tokenizer::tokenize(b"sip:user@server2.com:5060 something"),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: Some("sip".as_bytes().into()),
                    auth: Some(("user".as_bytes(), None).into()),
                    host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
                    params: vec![],
                    headers: None
                }
            )),
        );
    }

    #[test]
    fn tokenizer6() {
        assert_eq!(
            Tokenizer::tokenize(b"sip:user:password@server2.com:5060 something"),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: Some("sip".as_bytes().into()),
                    auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
                    host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
                    params: vec![],
                    headers: None
                }
            )),
        );
    }

    #[test]
    fn tokenizer7() {
        assert_eq!(
            Tokenizer::tokenize(b"sips:ss2.biloxi.example.com something"),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: Some("sips".as_bytes().into()),
                    auth: None,
                    host_with_port: ("ss2.biloxi.example.com".as_bytes(), None).into(),
                    params: vec![],
                    headers: None
                }
            )),
        );
    }

    #[test]
    fn tokenizer8() {
        assert_eq!(
            Tokenizer::tokenize(b"sips:ss2.biloxi.example.com;maddr=255.255.255.0 something"),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: Some("sips".as_bytes().into()),
                    auth: None,
                    host_with_port: ("ss2.biloxi.example.com".as_bytes(), None).into(),
                    params: vec![("maddr".as_bytes(), Some("255.255.255.0".as_bytes())).into()],
                    headers: None
                }
            )),
        );
    }

    #[test]
    fn tokenizer9() {
        assert_eq!(
            Tokenizer::tokenize(b"sips:client.biloxi.example.com:5061 something"),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: Some("sips".as_bytes().into()),
                    auth: None,
                    host_with_port: (
                        "client.biloxi.example.com".as_bytes(),
                        Some("5061".as_bytes())
                    )
                        .into(),
                    params: vec![],
                    headers: None
                }
            )),
        );
    }

    #[test]
    fn tokenizer10() {
        assert_eq!(
            Tokenizer::tokenize(
                b"sips:client.biloxi.example.com:5061;maddr=255.255.255.0;foo=192.0.2.201 something"
            ),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: Some("sips".as_bytes().into()),
                    auth: None,
                    host_with_port: (
                        "client.biloxi.example.com".as_bytes(),
                        Some("5061".as_bytes())
                    )
                        .into(),
                    params: vec![
                        ("maddr".as_bytes(), Some("255.255.255.0".as_bytes())).into(),
                        ("foo".as_bytes(), Some("192.0.2.201".as_bytes())).into()
                    ],
                    headers: None
                }
            )),
        );
    }

    #[test]
    fn tokenizer11() {
        assert_eq!(
            Tokenizer::tokenize(
                b"sips:client.biloxi.example.com:5061;maddr=255.255.255.0;foo=192.0.2.201;lr something"
            ),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: Some("sips".as_bytes().into()),
                    auth: None,
                    host_with_port: (
                        "client.biloxi.example.com".as_bytes(),
                        Some("5061".as_bytes())
                    )
                        .into(),
                    params: vec![
                        ("maddr".as_bytes(), Some("255.255.255.0".as_bytes())).into(),
                        ("foo".as_bytes(), Some("192.0.2.201".as_bytes())).into(),
                        ("lr".as_bytes(), None).into()
                    ],
                    headers: None
                }
            )),
        );
    }
}
