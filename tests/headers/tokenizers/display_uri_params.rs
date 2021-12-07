use rsip::{
    common::uri,
    headers::typed::{tokenizers::DisplayUriParamsTokenizer, Tokenize},
};

#[test]
fn tokenizer1() {
    assert_eq!(
        DisplayUriParamsTokenizer::tokenize("Alice <sip:alice@atlanta.example.com>;tag=9fxced76sl"),
        Ok(DisplayUriParamsTokenizer {
            display_name: Some("Alice"),
            uri: uri::Tokenizer {
                scheme: Some("sip".into()),
                auth: Some(uri::auth::Tokenizer::from(("alice", None,))),
                host_with_port: ("atlanta.example.com", None).into(),
                params: vec![],
                headers: None,
                ..Default::default()
            },
            params: vec![("tag", Some("9fxced76sl")).into()],
        })
    );
}

#[test]
fn tokenizer2() {
    assert_eq!(
        DisplayUriParamsTokenizer::tokenize("<sip:alice@atlanta.example.com>;tag=9fxced76sl"),
        Ok(DisplayUriParamsTokenizer {
            display_name: None,
            uri: uri::Tokenizer {
                scheme: Some("sip".into()),
                auth: Some(uri::auth::Tokenizer::from(("alice", None,))),
                host_with_port: ("atlanta.example.com", None).into(),
                params: vec![],
                headers: None,
                ..Default::default()
            },
            params: vec![("tag", Some("9fxced76sl")).into()],
        })
    );
}

#[test]
fn tokenizer3() {
    assert_eq!(
        DisplayUriParamsTokenizer::tokenize("sip:alice@atlanta.example.com;tag=9fxced76sl"),
        Ok(DisplayUriParamsTokenizer {
            display_name: None,
            uri: uri::Tokenizer {
                scheme: Some("sip".into()),
                auth: Some(uri::auth::Tokenizer::from(("alice", None,))),
                host_with_port: ("atlanta.example.com", None).into(),
                params: vec![],
                headers: None,
                ..Default::default()
            },
            params: vec![("tag", Some("9fxced76sl")).into()],
        })
    );
}

#[test]
fn tokenizer4() {
    assert_eq!(
        DisplayUriParamsTokenizer::tokenize("<sip:alice@atlanta.example.com>;expires=360"),
        Ok(DisplayUriParamsTokenizer {
            display_name: None,
            uri: uri::Tokenizer {
                scheme: Some("sip".into()),
                auth: Some(uri::auth::Tokenizer::from(("alice", None,))),
                host_with_port: ("atlanta.example.com", None).into(),
                params: vec![],
                headers: None,
                ..Default::default()
            },
            params: vec![("expires", Some("360")).into()],
        })
    );
}
