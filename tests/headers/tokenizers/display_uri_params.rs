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

// Used "+sip.instance" Contact param as an example for double-quote-enclosed param value
// for +sip.instance specification, see RFC 5626.
#[test]
fn tokenizer_with_instance() {
    assert_eq!(
        DisplayUriParamsTokenizer::tokenize(
            r#"<sip:alice@atlanta.example.com>;+sip.instance="<urn:uuid:9e397c26-8f87-4b02-9df1-987ddfe135c3>""#
        ),
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
            params: vec![(
                "+sip.instance",
                Some(r#""<urn:uuid:9e397c26-8f87-4b02-9df1-987ddfe135c3>""#)
            )
                .into()],
        })
    );
}
