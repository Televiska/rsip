use rsip::{common::uri, message::request::Tokenizer};

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"REGISTER sip:server.com SIP/2.0\r\n\r\n".as_ref()),
        Ok((
            b"".as_ref(),
            Tokenizer {
                method: b"REGISTER".as_ref().into(),
                uri: uri::Tokenizer {
                    schema: Some(b"sip".as_ref().into()),
                    auth: None,
                    host_with_port: (b"server.com".as_ref(), None).into(),
                    params: None,
                    headers: None
                },
                version: b"SIP/2.0".as_ref().into(),
                headers: vec![].into(),
                body: &[]
            }
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(
            concat!(
                "REGISTER sips:ss2.biloxi.example.com SIP/2.0\r\n",
                "Via: SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashd92\r\n",
                "Max-Forwards: 70\r\n",
                "From: Bob <sips:bob@biloxi.example.com>;tag=ja743ks76zlflH\r\n",
                "To: Bob <sips:bob@biloxi.example.com>\r\n",
                "Call-ID: 1j9FpLxk3uxtm8tn@biloxi.example.com\r\n",
                "CSeq: 2 REGISTER\r\n",
                "Contact: <sips:bob@client.biloxi.example.com>\r\n",
                "Authorization: Digest username=\"bob\", realm=\"atlanta.example.com\" nonce=\"ea9c8e88df84f1cec4341ae6cbe5a359\", opaque=\"\" uri=\"sips:ss2.biloxi.example.com\", response=\"dfe56131d1958046689d83306477ecc\"\r\n",
                "Content-Length: 0\r\n\r\n"
            ).as_bytes()
        ),
        Ok((
            b"".as_ref(),
            Tokenizer {
                method: b"REGISTER".as_ref().into(),
                uri: uri::Tokenizer {
                    schema: Some(b"sips".as_ref().into()),
                    auth: None,
                    host_with_port: (b"ss2.biloxi.example.com".as_ref(), None).into(),
                    params: None,
                    headers: None
                },
                version: b"SIP/2.0".as_ref().into(),
                headers: vec![
                    ("Via".as_bytes(), "SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashd92".as_bytes()).into(),
                    ("Max-Forwards".as_bytes(), "70".as_bytes()).into(),
                    ("From".as_bytes(), "Bob <sips:bob@biloxi.example.com>;tag=ja743ks76zlflH".as_bytes()).into(),
                    ("To".as_bytes(), "Bob <sips:bob@biloxi.example.com>".as_bytes()).into(),
                    ("Call-ID".as_bytes(), "1j9FpLxk3uxtm8tn@biloxi.example.com".as_bytes()).into(),
                    ("CSeq".as_bytes(), "2 REGISTER".as_bytes()).into(),
                    ("Contact".as_bytes(), "<sips:bob@client.biloxi.example.com>".as_bytes()).into(),
                    ("Authorization".as_bytes(), "Digest username=\"bob\", realm=\"atlanta.example.com\" nonce=\"ea9c8e88df84f1cec4341ae6cbe5a359\", opaque=\"\" uri=\"sips:ss2.biloxi.example.com\", response=\"dfe56131d1958046689d83306477ecc\"".as_bytes()).into(),
                    ("Content-Length".as_bytes(), "0".as_bytes()).into(),
                ],
                body: &[]
            }
        )),
    );

    assert!(Tokenizer::tokenize(b"SIP/2.0 401 Unauthorized\r\n\r\n".as_ref()).is_err());
}
