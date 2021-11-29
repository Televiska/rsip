use rsip::{
    common::uri,
    headers::typed::{via::Tokenizer, Tokenize},
};

#[test]
fn tokenizer1() {
    assert_eq!(
        Tokenizer::tokenize("SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashds7"),
        Ok(Tokenizer {
            version: ("2", "0").into(),
            transport: "TLS".into(),
            uri: uri::Tokenizer {
                scheme: None,
                auth: None,
                host_with_port: ("client.biloxi.example.com", Some("5061")).into(),
                params: vec![],
                headers: None,
                ..Default::default()
            },
            params: vec![("branch", Some("z9hG4bKnashds7")).into()],
        })
    );
}
