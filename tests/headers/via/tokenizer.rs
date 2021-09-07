use rsip::{
    common::uri,
    headers::typed::{via::Tokenizer, Tokenize},
};

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize("SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashds7"),
        Ok(Tokenizer {
            version: ("2".as_bytes(), "0".as_bytes()).into(),
            transport: "TLS".as_bytes().into(),
            uri: uri::Tokenizer {
                scheme: None,
                auth: None,
                host_with_port: (
                    "client.biloxi.example.com".as_bytes(),
                    Some("5061".as_bytes())
                )
                    .into(),
                params: vec![],
                headers: None
            },
            params: vec![("branch".as_bytes(), Some("z9hG4bKnashds7".as_bytes())).into()],
        })
    );
}
