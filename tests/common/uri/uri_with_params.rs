use rsip::common::uri::{
    self,
    param::{Maddr, Param, Tag},
    uri_with_params::{Tokenizer, UriWithParams},
    Scheme, Uri,
};
use std::convert::TryInto;

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(
            UriWithParams {
                uri: Uri {
                    scheme: Some(Scheme::Sips),
                    auth: None,
                    host_with_port: ("client.biloxi.example.com", Some(5061)).into(),
                    params: vec![Param::Other("s".into(), Some("2".into())),],
                    headers: Default::default()
                },
                params: vec![
                    Param::Maddr(Maddr::new("255.255.255.0")),
                    Param::Other("foo".into(), Some("192.0.2.201".into())),
                    Param::Lr,
                ],
            }
            .to_string(),
            String::from(
                "<sips:client.biloxi.example.com:5061;s=2>;maddr=255.255.255.0;foo=192.0.2.201;lr"
            )
        );
    }

    #[test]
    fn display2() {
        assert_eq!(
            UriWithParams {
                uri: Uri {
                    scheme: Some(Scheme::Sips),
                    auth: None,
                    host_with_port: ("client.biloxi.example.com", Some(5061)).into(),
                    params: vec![Param::Other("s".into(), Some("2".into())),],
                    headers: Default::default()
                },
                params: vec![],
            }
            .to_string(),
            String::from("<sips:client.biloxi.example.com:5061;s=2>")
        );
    }

    #[test]
    fn display3() {
        let tag = Tag::default();
        assert_eq!(
            UriWithParams {
                uri: Uri {
                    scheme: Some(Scheme::Tel),
                    auth: None,
                    host_with_port: "+12124567890".try_into().unwrap(),
                    params: Default::default(),
                    headers: Default::default()
                },
                params: vec![Param::Tag(tag.clone())],
            }
            .to_string(),
            format!("<tel:+12124567890>;tag={}", tag)
        )
    }
}

mod parser {
    use super::*;

    #[test]
    fn parser1_u8() {
        assert_eq!(
            Tokenizer {
                uri: uri::Tokenizer {
                    scheme: Some("sips".as_bytes().into()),
                    auth: None,
                    host_with_port: (
                        "client.biloxi.example.com".as_bytes(),
                        Some("5061".as_bytes())
                    )
                        .into(),
                    params: vec![("s".as_bytes(), Some("2".as_bytes())).into()],
                    headers: None,
                    ..Default::default()
                },
                params: vec![
                    ("maddr".as_bytes(), Some("255.255.255.0".as_bytes())).into(),
                    ("foo".as_bytes(), Some("192.0.2.201".as_bytes())).into(),
                    ("lr".as_bytes(), None).into()
                ],
                ..Default::default()
            }
            .try_into(),
            Ok(UriWithParams {
                uri: Uri {
                    scheme: Some(Scheme::Sips),
                    auth: None,
                    host_with_port: ("client.biloxi.example.com", Some(5061)).into(),
                    params: vec![Param::Other("s".into(), Some("2".into())),],
                    headers: Default::default()
                },
                params: vec![
                    Param::Maddr(Maddr::new("255.255.255.0")),
                    Param::Other("foo".into(), Some("192.0.2.201".into())),
                    Param::Lr,
                ],
            })
        );
    }

    #[test]
    fn parser1_str() {
        assert_eq!(
            Tokenizer {
                uri: uri::Tokenizer {
                    scheme: Some("sips".into()),
                    auth: None,
                    host_with_port: ("client.biloxi.example.com", Some("5061")).into(),
                    params: vec![("s", Some("2")).into()],
                    headers: None,
                    ..Default::default()
                },
                params: vec![
                    ("maddr", Some("255.255.255.0")).into(),
                    ("foo", Some("192.0.2.201")).into(),
                    ("lr", None).into()
                ],
                ..Default::default()
            }
            .try_into(),
            Ok(UriWithParams {
                uri: Uri {
                    scheme: Some(Scheme::Sips),
                    auth: None,
                    host_with_port: ("client.biloxi.example.com", Some(5061)).into(),
                    params: vec![Param::Other("s".into(), Some("2".into())),],
                    headers: Default::default()
                },
                params: vec![
                    Param::Maddr(Maddr::new("255.255.255.0")),
                    Param::Other("foo".into(), Some("192.0.2.201".into())),
                    Param::Lr,
                ],
            })
        );
    }
}

mod tokenizer {
    use super::*;

    #[test]
    fn tokenizer1_u8() {
        assert_eq!(
            Tokenizer::tokenize(
                "sips:client.biloxi.example.com:5061;maddr=255.255.255.0;foo=192.0.2.201;lr,foo=1"
                    .as_bytes()
            ),
            Ok((
                "foo=1".as_bytes(),
                Tokenizer {
                    uri: uri::Tokenizer {
                        scheme: Some("sips".as_bytes().into()),
                        auth: None,
                        host_with_port: (
                            "client.biloxi.example.com".as_bytes(),
                            Some("5061".as_bytes())
                        )
                            .into(),
                        params: vec![],
                        headers: None,
                        ..Default::default()
                    },
                    params: vec![
                        ("maddr".as_bytes(), Some("255.255.255.0".as_bytes())).into(),
                        ("foo".as_bytes(), Some("192.0.2.201".as_bytes())).into(),
                        ("lr".as_bytes(), None).into()
                    ],
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer1_str() {
        assert_eq!(
            Tokenizer::tokenize(
                "sips:client.biloxi.example.com:5061;maddr=255.255.255.0;foo=192.0.2.201;lr,foo=1"
            ),
            Ok((
                "foo=1",
                Tokenizer {
                    uri: uri::Tokenizer {
                        scheme: Some("sips".into()),
                        auth: None,
                        host_with_port: ("client.biloxi.example.com", Some("5061")).into(),
                        params: vec![],
                        headers: None,
                        ..Default::default()
                    },
                    params: vec![
                        ("maddr", Some("255.255.255.0")).into(),
                        ("foo", Some("192.0.2.201")).into(),
                        ("lr", None).into()
                    ],
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer2_u8() {
        assert_eq!(
            Tokenizer::tokenize("<sip:alice@atlanta.example.com;s=2>;level=low,foo=1".as_bytes()),
            Ok((
                "foo=1".as_bytes(),
                Tokenizer {
                    uri: uri::Tokenizer {
                        scheme: Some("sip".as_bytes().into()),
                        auth: Some(("alice".as_bytes(), None).into()),
                        host_with_port: ("atlanta.example.com".as_bytes(), None).into(),
                        params: vec![("s".as_bytes(), Some("2".as_bytes())).into()],
                        headers: None,
                        ..Default::default()
                    },
                    params: vec![("level".as_bytes(), Some("low".as_bytes())).into()],
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer2_str() {
        assert_eq!(
            Tokenizer::tokenize("<sip:alice@atlanta.example.com;s=2>;level=low,foo=1"),
            Ok((
                "foo=1",
                Tokenizer {
                    uri: uri::Tokenizer {
                        scheme: Some("sip".into()),
                        auth: Some(("alice", None).into()),
                        host_with_port: ("atlanta.example.com", None).into(),
                        params: vec![("s", Some("2")).into()],
                        headers: None,
                        ..Default::default()
                    },
                    params: vec![("level", Some("low")).into()],
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer3_str() {
        let tag: String = Tag::default().into();
        let input = format!("<tel:+12124567890>;tag={}", tag);
        assert_eq!(
            Tokenizer::tokenize(input.as_str()),
            Ok((
                "",
                Tokenizer {
                    uri: uri::Tokenizer {
                        scheme: Some("tel".into()),
                        auth: None,
                        host_with_port: ("+12124567890", None).into(),
                        params: vec![],
                        headers: None,
                        ..Default::default()
                    },
                    params: vec![("tag", Some(tag.as_str())).into()],
                    ..Default::default()
                }
            ))
        );
    }
}
