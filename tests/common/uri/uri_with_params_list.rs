use rsip::common::uri::{
    self,
    param::{Maddr, Param},
    uri_with_params::{self, UriWithParams},
    uri_with_params_list::{Tokenizer, UriWithParamsList},
    Scheme, Uri,
};

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(
            UriWithParamsList(vec![
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
                },
                UriWithParams {
                    uri: Uri {
                        scheme: Some(Scheme::Other("https".into())),
                        auth: None,
                        host_with_port: ("www.example.com", Option::<u16>::None).into(),
                        params: vec![Param::Other("foo".into(), Some("bar".into())),],
                        headers: Default::default()
                    },
                    params: vec![Param::Other("test".into(), None)],
                }
            ])
            .to_string(),
            String::from(concat!(
                "<sips:client.biloxi.example.com:5061;s=2>;maddr=255.255.255.0;foo=192.0.2.201;lr",
                ",",
                "<https://www.example.com;foo=bar>;test",
            ))
        );
    }
}

mod tokenizer {
    use super::*;

    #[test]
    fn tokenizer1_u8() {
        assert_eq!(
            Tokenizer::tokenize(
                concat!(
                    "sips:client.biloxi.example.com:5061;maddr=255.255.255.0;foo=192.0.2.201;lr",
                    ",",
                    "<sip:alice@atlanta.example.com;s=2>;level=low"
                )
                .as_bytes()
            ),
            Ok((
                "".as_bytes(),
                Tokenizer {
                    values: vec![
                        uri_with_params::Tokenizer {
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
                        },
                        uri_with_params::Tokenizer {
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
                    ],
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer2_u8() {
        assert_eq!(
            Tokenizer::tokenize(
                concat!(
                    "sips:client.biloxi.example.com:5061;maddr=255.255.255.0;foo=192.0.2.201;lr",
                    ",",
                    "<sip:alice.smith@atlanta.example.com;s=2>;level=low"
                )
                .as_bytes()
            ),
            Ok((
                "".as_bytes(),
                Tokenizer {
                    values: vec![
                        uri_with_params::Tokenizer {
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
                        },
                        uri_with_params::Tokenizer {
                            uri: uri::Tokenizer {
                                scheme: Some("sip".as_bytes().into()),
                                auth: Some(("alice.smith".as_bytes(), None).into()),
                                host_with_port: ("atlanta.example.com".as_bytes(), None).into(),
                                params: vec![("s".as_bytes(), Some("2".as_bytes())).into()],
                                headers: None,
                                ..Default::default()
                            },
                            params: vec![("level".as_bytes(), Some("low".as_bytes())).into()],
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer3_u8() {
        assert_eq!(
            Tokenizer::tokenize(
                concat!(
                    "sips:localhost:5061;lr",
                    ",",
                    "<sip:alice@atlanta.example.com;s=2>;level=low"
                )
                .as_bytes()
            ),
            Ok((
                "".as_bytes(),
                Tokenizer {
                    values: vec![
                        uri_with_params::Tokenizer {
                            uri: uri::Tokenizer {
                                scheme: Some("sips".as_bytes().into()),
                                auth: None,
                                host_with_port: (
                                    "localhost".as_bytes(),
                                    Some("5061".as_bytes())
                                )
                                    .into(),
                                params: vec![],
                                headers: None,
                                ..Default::default()
                            },
                            params: vec![
                                ("lr".as_bytes(), None).into()
                            ],
                            ..Default::default()
                        },
                        uri_with_params::Tokenizer {
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
                    ],
                    ..Default::default()
                }
            )),
        );
    }
}
