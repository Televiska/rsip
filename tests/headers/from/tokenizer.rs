use rsip::{
    common::uri,
    headers::typed::{from::Tokenizer, Tokenize},
};

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize("Alice <sip:alice@atlanta.example.com>;tag=9fxced76sl"),
        Ok(Tokenizer {
            display_name: Some("Alice"),
            uri: uri::Tokenizer {
                scheme: Some("sip".as_bytes().into()),
                auth: Some(uri::auth::Tokenizer {
                    user: "alice".as_bytes(),
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
                scheme: Some("sip".as_bytes().into()),
                auth: Some(uri::auth::Tokenizer {
                    user: "alice".as_bytes(),
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
                scheme: Some("sip".as_bytes().into()),
                auth: Some(uri::auth::Tokenizer {
                    user: "alice".as_bytes(),
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
