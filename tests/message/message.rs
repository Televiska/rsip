use rsip::{
    common::{self, uri},
    headers::*,
    message::{request, response, sip_message::Tokenizer},
    Request, Response, SipMessage,
};
use std::convert::TryFrom;

#[test]
fn methods() {
    let uri = uri::Uri {
        scheme: Some(uri::scheme::Scheme::Sips),
        auth: None,
        host_with_port: uri::HostWithPort {
            host: uri::Host::Domain("ss2.biloxi.example.com".into()),
            port: None,
        },
        params: vec![],
        headers: vec![].into(),
    };
    let headers: rsip::Headers = vec![
                Via::new("SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashd92").into(),
                MaxForwards::new("70").into(),
                From::new("Bob <sips:bob@biloxi.example.com>;tag=ja743ks76zlflH").into(),
                To::new("Bob <sips:bob@biloxi.example.com>").into(),
                CallId::new("1j9FpLxk3uxtm8tn@biloxi.example.com").into(),
                CSeq::new("2 REGISTER").into(),
                Contact::new("<sips:bob@client.biloxi.example.com>").into(),
                Authorization::new("Digest username=\"bob\", realm=\"atlanta.example.com\" nonce=\"ea9c8e88df84f1cec4341ae6cbe5a359\", opaque=\"\" uri=\"sips:ss2.biloxi.example.com\", response=\"dfe56131d1958046689d83306477ecc\"").into(),
                ContentLength::new("0").into(),
            ].into();
    let sip_message = SipMessage::Request(Request {
        method: common::method::Method::Register,
        uri: uri.clone(),
        version: common::version::Version::V2,
        headers: headers.clone(),
        body: vec![1, 2, 3],
    });

    assert_eq!(sip_message.version(), &common::version::Version::V2);
    assert_eq!(sip_message.body(), &vec![1, 2, 3]);
    //TODO: how do I test mut signatures (mut_body & mut_headers) ?
    assert_eq!(rsip::message::HasHeaders::headers(&sip_message), &headers);
}

#[test]
fn bytes() {
    assert_eq!(
        Into::<bytes::Bytes>::into(SipMessage::Response(Response {
            status_code: common::StatusCode::Unauthorized,
            version: common::Version::V2,
            headers: vec![].into(),
            body: vec![]
        })),
        bytes::Bytes::from(String::from("SIP/2.0 401 Unauthorized\r\n\r\n"))
    );
}

#[test]
fn display() {
    assert_eq!(
        SipMessage::Response(Response {
            status_code: common::StatusCode::Unauthorized,
            version: common::Version::V2,
            headers: vec![].into(),
            body: vec![]
        })
        .to_string(),
        String::from("SIP/2.0 401 Unauthorized\r\n\r\n")
    );
}

#[test]
fn parser() {
    assert_eq!(
        SipMessage::try_from("REGISTER sip:server.com SIP/2.0\r\n\r\n".as_bytes()),
        Ok(SipMessage::Request(Request {
            method: common::method::Method::Register,
            uri: uri::Uri {
                scheme: Some(uri::scheme::Scheme::Sip),
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
                scheme: Some(uri::scheme::Scheme::Sips),
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

    assert_eq!(
        SipMessage::try_from(
            concat!(
                "INVITE sip:1004@server.test.com:5060 SIP/2.0\r\n",
                "Accept: application/sdp\r\n",
                "Content-Type: application/sdp\r\n",
                "Content-Length: 5\r\n\r\n",
                "v=0\r\n"
            )
            .as_bytes()
        ),
        Ok(SipMessage::Request(Request {
            method: common::method::Method::Invite,
            uri: uri::Uri {
                scheme: Some(uri::scheme::Scheme::Sip),
                auth: Some(uri::Auth {
                    user: "1004".into(),
                    password: None
                }),
                host_with_port: uri::HostWithPort {
                    host: uri::Host::Domain("server.test.com".into()),
                    port: Some(5060.into())
                },
                params: vec![],
                headers: vec![].into()
            },
            version: common::version::Version::V2,
            headers: vec![
                Accept::new("application/sdp").into(),
                ContentType::new("application/sdp").into(),
                ContentLength::new("5").into(),
            ]
            .into(),
            body: vec![b'v', b'=', b'0', b'\r', b'\n']
        }))
    );

    assert_eq!(
        SipMessage::try_from(
            concat!(
                "INVITE sip:1004@server.test.com:5060 SIP/2.0\r\n",
                "Accept: application/sdp\r\n",
                "Content-Type: application/sdp\r\n",
                "Content-Length: 16\r\n\r\n",
                "a=fmtp:96 0-15\r\n"
            )
            .as_bytes()
        ),
        Ok(SipMessage::Request(Request {
            method: common::method::Method::Invite,
            uri: uri::Uri {
                scheme: Some(uri::scheme::Scheme::Sip),
                auth: Some(uri::Auth {
                    user: "1004".into(),
                    password: None
                }),
                host_with_port: uri::HostWithPort {
                    host: uri::Host::Domain("server.test.com".into()),
                    port: Some(5060.into())
                },
                params: vec![],
                headers: vec![].into()
            },
            version: common::version::Version::V2,
            headers: vec![
                Accept::new("application/sdp").into(),
                ContentType::new("application/sdp").into(),
                ContentLength::new("16").into(),
            ]
            .into(),
            body: vec![
                b'a', b'=', b'f', b'm', b't', b'p', b':', b'9', b'6', b' ', b'0', b'-', b'1', b'5',
                b'\r', b'\n',
            ]
        }))
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
                    scheme: Some("sip".as_bytes().into()),
                    auth: None,
                    host_with_port: ("server.com".as_bytes(), None).into(),
                    params: vec![],
                    headers: None
                },
                version: ("2".as_bytes(), "0".as_bytes()).into(),
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
                    scheme: Some("sips".as_bytes().into()),
                    auth: None,
                    host_with_port: ("ss2.biloxi.example.com".as_bytes(), None).into(),
                    params: vec![],
                    headers: None
                },
                version: ("2".as_bytes(), "0".as_bytes()).into(),
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
                version: ("2".as_bytes(), "0".as_bytes()).into(),
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
                version: ("2".as_bytes(), "0".as_bytes()).into(),
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
