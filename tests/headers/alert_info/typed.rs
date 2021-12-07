use rsip::{
    common::uri::{self, param::Maddr, Param, Scheme, Uri, UriWithParams, UriWithParamsList},
    headers::typed::{alert_info::Tokenizer, tokenizers::UriWithParamsTokenizer, AlertInfo},
};
use std::convert::TryInto;

validate_typed_header_trait!(AlertInfo);

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(
            AlertInfo(UriWithParamsList(vec![
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
            ]))
            .to_string(),
            String::from(concat!(
                "<sips:client.biloxi.example.com:5061;s=2>;maddr=255.255.255.0;foo=192.0.2.201;lr",
                ",",
                "<https:www.example.com;foo=bar>;test",
            ))
        );
    }
}

mod try_from_tokenizer {
    use super::*;

    #[test]
    fn try_from_1() -> Result<(), rsip::Error> {
        assert_eq!(
            Tokenizer {
                values: vec![
                    UriWithParamsTokenizer {
                        uri: uri::Tokenizer {
                            scheme: Some("sips".into()),
                            host_with_port: ("client.biloxi.example.com", Some("5061")).into(),
                            params: vec![("s", Some("2")).into()],
                            ..Default::default()
                        },
                        params: vec![
                            ("maddr", Some("255.255.255.0")).into(),
                            ("foo", Some("192.0.2.201")).into(),
                            ("lr", None).into(),
                        ],
                        ..Default::default()
                    },
                    UriWithParamsTokenizer {
                        uri: uri::Tokenizer {
                            scheme: Some("https".into()),
                            host_with_port: ("www.example.com", None).into(),
                            params: vec![("foo", Some("bar")).into()],
                            ..Default::default()
                        },
                        params: vec![("test", None).into(),],
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
            .try_into(),
            Ok(AlertInfo(UriWithParamsList(vec![
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
            ])))
        );

        Ok(())
    }
}
