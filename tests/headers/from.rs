use rsip::headers::header::Tokenize;
use rsip::{
    common::uri,
    headers::header::from::{self, Tokenizer},
};
use std::convert::TryInto;

validate_untyped_header_trait!(from, From);

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize("Alice <sip:alice@atlanta.example.com>;tag=9fxced76sl"),
        Ok(Tokenizer {
            display_name: Some("Alice"),
            uri: uri::Tokenizer {
                schema: Some("sip".as_bytes().into()),
                auth: Some(uri::auth::Tokenizer {
                    username: "alice".as_bytes(),
                    password: None
                }),
                host_with_port: ("atlanta.example.com".as_bytes(), None).into(),
                params: vec![],
                headers: None
            },
            params: vec![("tag".as_bytes(), Some("9fxced76sl".as_bytes())).into()],
        })
    );

    assert_eq!(
        Tokenizer::tokenize("<sip:alice@atlanta.example.com>;tag=9fxced76sl"),
        Ok(Tokenizer {
            display_name: None,
            uri: uri::Tokenizer {
                schema: Some("sip".as_bytes().into()),
                auth: Some(uri::auth::Tokenizer {
                    username: "alice".as_bytes(),
                    password: None
                }),
                host_with_port: ("atlanta.example.com".as_bytes(), None).into(),
                params: vec![],
                headers: None
            },
            params: vec![("tag".as_bytes(), Some("9fxced76sl".as_bytes())).into()],
        })
    );

    assert_eq!(
        Tokenizer::tokenize("sip:alice@atlanta.example.com;tag=9fxced76sl"),
        Ok(Tokenizer {
            display_name: None,
            uri: uri::Tokenizer {
                schema: Some("sip".as_bytes().into()),
                auth: Some(uri::auth::Tokenizer {
                    username: "alice".as_bytes(),
                    password: None
                }),
                host_with_port: ("atlanta.example.com".as_bytes(), None).into(),
                params: vec![],
                headers: None
            },
            params: vec![("tag".as_bytes(), Some("9fxced76sl".as_bytes())).into()],
        })
    );
}

mod typed {
    use super::*;

    validate_untyped_header_trait!(from, From);

    #[test]
    fn display() {
        assert_eq!(
            format!(
                "{}",
                from::typed::From {
                    display_name: Some("Alice".into()),
                    uri: uri::Uri {
                        schema: Some(uri::Schema::Sip),
                        auth: Some(uri::Auth {
                            username: "alice".into(),
                            password: None
                        }),
                        host_with_port: uri::HostWithPort::from("atlanta.example.com"),
                        params: vec![],
                        headers: vec![].into()
                    },
                    params: vec![uri::Param::Tag(uri::param::Tag::new("9fxced76sl"))]
                }
            ),
            String::from("Alice <sip:alice@atlanta.example.com>;tag=9fxced76sl")
        )
    }

    #[test]
    fn from_tokenizer() {
        assert_eq!(
            Tokenizer {
                display_name: Some("Alice"),
                uri: uri::Tokenizer {
                    schema: Some("sip".as_bytes().into()),
                    auth: Some(uri::auth::Tokenizer {
                        username: "alice".as_bytes(),
                        password: None
                    }),
                    host_with_port: ("atlanta.example.com".as_bytes(), None).into(),
                    params: vec![],
                    headers: None
                },
                params: vec![("tag".as_bytes(), Some("9fxced76sl".as_bytes())).into()],
            }
            .try_into(),
            Ok(from::typed::From {
                display_name: Some("Alice".into()),
                uri: uri::Uri {
                    schema: Some(uri::Schema::Sip),
                    auth: Some(uri::Auth {
                        username: "alice".into(),
                        password: None
                    }),
                    host_with_port: uri::HostWithPort::from("atlanta.example.com"),
                    params: vec![],
                    headers: vec![].into()
                },
                params: vec![uri::Param::Tag(uri::param::Tag::new("9fxced76sl"))]
            })
        );
    }
}
