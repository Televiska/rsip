use rsip::{
    common::uri,
    headers::typed::{contact::Tokenizer, Tokenize},
};

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenize::tokenize("Alice <sip:alice@atlanta.example.com>;tag=9fxced76sl"),
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
        Tokenize::tokenize("<sip:alice@atlanta.example.com>;expires=360"),
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
            params: vec![("expires".as_bytes(), Some("360".as_bytes())).into()],
        })
    );

    assert_eq!(
        Tokenize::tokenize("sip:alice@atlanta.example.com;tag=9fxced76sl"),
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