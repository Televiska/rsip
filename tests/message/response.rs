use rsip::{
    common,
    headers::*,
    message::response::{Response, Tokenizer},
};
use std::convert::TryFrom;

#[test]
fn methods() {
    let headers: rsip::headers::Headers = vec![
                Via::new("SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashds7;received=192.0.2.201").into(),
                From::new("Bob <sips:bob@biloxi.example.com>;tag=a73kszlfl").into(),
                To::new("Bob <sips:bob@biloxi.example.com>;tag=1410948204").into(),
                CallId::new("1j9FpLxk3uxtm8tn@biloxi.example.com").into(),
                CSeq::new("1 REGISTER").into(),
                WwwAuthenticate::new("Digest realm=\"atlanta.example.com\", qop=\"auth\", nonce=\"ea9c8e88df84f1cec4341ae6cbe5a359\", opaque=\"\", stale=FALSE, algorithm=MD5").into(),
                ContentLength::new("0").into(),
            ].into();
    let response = Response {
        status_code: common::StatusCode::Unauthorized,
        version: common::Version::V2,
        headers: headers.clone(),
        body: vec![1, 2, 3],
    };

    assert_eq!(response.status_code(), &common::StatusCode::Unauthorized);
    assert_eq!(response.version(), &common::version::Version::V2);
    assert_eq!(response.body(), &vec![1, 2, 3]);
    //TODO: how do I test mut signatures (mut_body & mut_headers) ?
    assert_eq!(rsip::message::HasHeaders::headers(&response), &headers);
}

#[test]
fn bytes() {
    assert_eq!(
        Into::<bytes::Bytes>::into(Response {
            status_code: common::StatusCode::Unauthorized,
            version: common::Version::V2,
            headers: vec![].into(),
            body: vec![]
        }),
        bytes::Bytes::from(String::from("SIP/2.0 401 Unauthorized\r\n\r\n"))
    );
}

#[test]
fn display() {
    assert_eq!(
        Response {
            status_code: common::StatusCode::Unauthorized,
            version: common::Version::V2,
            headers: vec![].into(),
            body: vec![]
        }
        .to_string(),
        String::from("SIP/2.0 401 Unauthorized\r\n\r\n")
    );

    assert_eq!(
        Response {
            status_code: common::StatusCode::Unauthorized,
            version: common::Version::V2,
            headers: vec![
                Via::new("SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashds7;received=192.0.2.201").into(),
                From::new("Bob <sips:bob@biloxi.example.com>;tag=a73kszlfl").into(),
                To::new("Bob <sips:bob@biloxi.example.com>;tag=1410948204").into(),
                CallId::new("1j9FpLxk3uxtm8tn@biloxi.example.com").into(),
                CSeq::new("1 REGISTER").into(),
                WwwAuthenticate::new("Digest realm=\"atlanta.example.com\", qop=\"auth\", nonce=\"ea9c8e88df84f1cec4341ae6cbe5a359\", opaque=\"\", stale=FALSE, algorithm=MD5").into(),
                ContentLength::new("0").into(),
            ].into(),
            body: vec![]
        }
        .to_string(),
        concat!(
           "SIP/2.0 401 Unauthorized\r\n",
           "Via: SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashds7;received=192.0.2.201\r\n",
           "From: Bob <sips:bob@biloxi.example.com>;tag=a73kszlfl\r\n",
           "To: Bob <sips:bob@biloxi.example.com>;tag=1410948204\r\n",
           "Call-ID: 1j9FpLxk3uxtm8tn@biloxi.example.com\r\n",
           "CSeq: 1 REGISTER\r\n",
           "WWW-Authenticate: Digest realm=\"atlanta.example.com\", qop=\"auth\", nonce=\"ea9c8e88df84f1cec4341ae6cbe5a359\", opaque=\"\", stale=FALSE, algorithm=MD5\r\n",
           "Content-Length: 0\r\n\r\n"
        ).to_string()
    );
}

#[test]
fn parser() {
    assert_eq!(
        Response::try_from("SIP/2.0 401 Unauthorized\r\n\r\n".as_bytes()),
        Ok(Response {
            status_code: common::StatusCode::Unauthorized,
            version: common::Version::V2,
            headers: vec![].into(),
            body: vec![]
        }),
    );

    assert_eq!(
        Response::try_from(
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
        Ok(Response {
            status_code: common::StatusCode::Unauthorized,
            version: common::Version::V2,
            headers: vec![
                Via::new("SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashds7;received=192.0.2.201").into(),
                From::new("Bob <sips:bob@biloxi.example.com>;tag=a73kszlfl").into(),
                To::new("Bob <sips:bob@biloxi.example.com>;tag=1410948204").into(),
                CallId::new("1j9FpLxk3uxtm8tn@biloxi.example.com").into(),
                CSeq::new("1 REGISTER").into(),
                WwwAuthenticate::new("Digest realm=\"atlanta.example.com\", qop=\"auth\", nonce=\"ea9c8e88df84f1cec4341ae6cbe5a359\", opaque=\"\", stale=FALSE, algorithm=MD5").into(),
                ContentLength::new("0").into(),
            ].into(),
            body: vec![]
        }),
    );
}

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize("SIP/2.0 401 Unauthorized\r\n\r\n".as_bytes()),
        Ok((
            "".as_bytes(),
            Tokenizer {
                version: "2".as_bytes().into(),
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
            "".as_bytes(),
            Tokenizer {
                version: "2".as_bytes().into(),
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

    assert!(Tokenizer::tokenize("REGISTER sip:server.com SIP/2.0\r\n\r\n".as_bytes()).is_err());
}
