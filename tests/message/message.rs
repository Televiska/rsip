use rsip::{
    common::{self, uri},
    headers::header::*,
    message::{request, response, Tokenizer},
    Request, Response, SipMessage,
};
use std::convert::TryFrom;

#[test]
fn parser() {
    assert_eq!(
        SipMessage::try_from("REGISTER sip:server.com SIP/2.0\r\n\r\n".as_bytes()),
        Ok(SipMessage::Request(Request {
            method: common::method::Method::Register,
            uri: uri::Uri {
                schema: Some(uri::schema::Schema::Sip),
                auth: None,
                host_with_port: uri::HostWithPort {
                    host: uri::Host::Domain("server.com".into()),
                    port: None
                },
                params: vec![],
                headers: vec![].into()
            },
            version: common::version::Version::V2,
            headers: vec![].into(),
            body: vec![]
        })),
    );

    assert_eq!(
        SipMessage::try_from(
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
        Ok(SipMessage::Request(Request {
            method: common::method::Method::Register,
            uri: uri::Uri {
                schema: Some(uri::schema::Schema::Sips),
                auth: None,
                host_with_port: uri::HostWithPort {
                    host: uri::Host::Domain("ss2.biloxi.example.com".into()),
                    port: None
                },
                params: vec![],
                headers: vec![].into()
            },
            version: common::version::Version::V2,
            headers: vec![
                Via::new("SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashd92").into(),
                MaxForwards::new("70").into(),
                From::new("Bob <sips:bob@biloxi.example.com>;tag=ja743ks76zlflH").into(),
                To::new("Bob <sips:bob@biloxi.example.com>").into(),
                CallId::new("1j9FpLxk3uxtm8tn@biloxi.example.com").into(),
                CSeq::new("2 REGISTER").into(),
                Contact::new("<sips:bob@client.biloxi.example.com>").into(),
                Authorization::new("Digest username=\"bob\", realm=\"atlanta.example.com\" nonce=\"ea9c8e88df84f1cec4341ae6cbe5a359\", opaque=\"\" uri=\"sips:ss2.biloxi.example.com\", response=\"dfe56131d1958046689d83306477ecc\"").into(),
                ContentLength::new("0").into(),
            ].into(),
            body: vec![]
        })),
    );

    assert_eq!(
        SipMessage::try_from("SIP/2.0 401 Unauthorized\r\n\r\n".as_bytes()),
        Ok(SipMessage::Response(Response {
            status_code: common::StatusCode::Unauthorized,
            version: common::Version::V2,
            headers: vec![].into(),
            body: vec![]
        })),
    );

    assert_eq!(
        SipMessage::try_from(
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
        Ok(SipMessage::Response(Response {
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
        })),
    );
}

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize("REGISTER sip:server.com SIP/2.0\r\n\r\n".as_bytes()),
        Ok((
            "".as_bytes(),
            Tokenizer::Request(request::Tokenizer {
                method: "REGISTER".as_bytes().into(),
                uri: uri::Tokenizer {
                    schema: Some("sip".as_bytes().into()),
                    auth: None,
                    host_with_port: ("server.com".as_bytes(), None).into(),
                    params: None,
                    headers: None
                },
                version: "SIP/2.0".as_bytes().into(),
                headers: vec![].into(),
                body: &[]
            })
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
            "".as_bytes(),
            Tokenizer::Request(request::Tokenizer {
                method: "REGISTER".as_bytes().into(),
                uri: uri::Tokenizer {
                    schema: Some("sips".as_bytes().into()),
                    auth: None,
                    host_with_port: ("ss2.biloxi.example.com".as_bytes(), None).into(),
                    params: None,
                    headers: None
                },
                version: "SIP/2.0".as_bytes().into(),
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
            })
        )),
    );

    assert_eq!(
        Tokenizer::tokenize("SIP/2.0 401 Unauthorized\r\n\r\n".as_bytes()),
        Ok((
            "".as_bytes(),
            Tokenizer::Response(response::Tokenizer {
                version: "SIP/2.0".as_bytes().into(),
                status_code: ("401".as_bytes(), "Unauthorized".as_bytes()).into(),
                headers: vec![].into(),
                body: &[]
            })
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
            Tokenizer::Response(response::Tokenizer {
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
            })
        )),
    );
}
