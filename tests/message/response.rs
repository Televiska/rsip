use rsip::message::response::Tokenizer;

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"SIP/2.0 401 Unauthorized\r\n\r\n".as_ref()),
        Ok((
            b"".as_ref(),
            Tokenizer {
                version: "SIP/2.0".as_bytes().into(),
                status_code: ("401".as_bytes(), "Unauthorized".as_bytes()).into(),
                headers: vec![].into(),
                body: &[]
            }
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(
            concat!(
               "SIP/2.0 401 Unauthorized\r\n",
               "Via: SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashds7;received=192.0.2.201\r\n",
               "From: Bob <sips:bob@biloxi.example.com>;tag=a73kszlfl\r\n",
               "To: Bob <sips:bob@biloxi.example.com>;tag=1410948204\r\n",
               "Call-ID: 1j9FpLxk3uxtm8tn@biloxi.example.com\r\n",
               "CSeq: 1 REGISTER\r\n",
               "WWW-Authenticate: Digest realm=\"atlanta.example.com\", qop=\"auth\", nonce=\"ea9c8e88df84f1cec4341ae6cbe5a359\", opaque=\"\", stale=FALSE, algorithm=MD5\r\n",
               "Content-Length: 0\r\n\r\n"
            ).as_bytes()
        ),
        Ok((
            b"".as_ref(),
            Tokenizer {
                version: "SIP/2.0".as_bytes().into(),
                status_code: ("401".as_bytes(), "Unauthorized".as_bytes()).into(),
                headers: vec![
                    ("Via".as_bytes(), "SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashds7;received=192.0.2.201".as_bytes()).into(),
                    ("From".as_bytes(), "Bob <sips:bob@biloxi.example.com>;tag=a73kszlfl".as_bytes()).into(),
                    ("To".as_bytes(), "Bob <sips:bob@biloxi.example.com>;tag=1410948204".as_bytes()).into(),
                    ("Call-ID".as_bytes(), "1j9FpLxk3uxtm8tn@biloxi.example.com".as_bytes()).into(),
                    ("CSeq".as_bytes(), "1 REGISTER".as_bytes()).into(),
                    ("WWW-Authenticate".as_bytes(), "Digest realm=\"atlanta.example.com\", qop=\"auth\", nonce=\"ea9c8e88df84f1cec4341ae6cbe5a359\", opaque=\"\", stale=FALSE, algorithm=MD5".as_bytes()).into(),
                    ("Content-Length".as_bytes(), "0".as_bytes()).into(),
                ],
                body: &[]
            }
        )),
    );

    assert!(Tokenizer::tokenize(b"REGISTER sip:server.com SIP/2.0\r\n\r\n".as_ref()).is_err());
}
