use rsip::{
    common::{self, uri},
    headers::*,
    message::request::{Request, Tokenizer},
};
use std::convert::TryFrom;

#[test]
fn methods() {
    let uri = uri::Uri {
        schema: Some(uri::schema::Schema::Sips),
        auth: None,
        host_with_port: uri::HostWithPort {
            host: uri::Host::Domain("ss2.biloxi.example.com".into()),
            port: None,
        },
        params: vec![],
        headers: vec![].into(),
    };
    let headers: rsip::headers::Headers = vec![
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
    let request = Request {
        method: common::method::Method::Register,
        uri: uri.clone(),
        version: common::version::Version::V2,
        headers: headers.clone(),
        body: vec![1, 2, 3],
    };

    assert_eq!(request.method(), &common::method::Method::Register);
    assert_eq!(request.uri(), &uri);
    assert_eq!(request.version(), &common::version::Version::V2);
    assert_eq!(request.body(), &vec![1, 2, 3]);
    //TODO: how do I test mut signatures (mut_body & mut_headers) ?
    assert_eq!(rsip::message::HasHeaders::headers(&request), &headers);
}

#[test]
fn bytes() {
    assert_eq!(
        Into::<bytes::Bytes>::into(Request {
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
        }),
        bytes::Bytes::from(String::from("REGISTER sip:server.com SIP/2.0\r\n\r\n"))
    );
}

#[test]
fn display() {
    assert_eq!(
        Request {
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
        }
        .to_string(),
        String::from("REGISTER sip:server.com SIP/2.0\r\n\r\n")
    );

    assert_eq!(
        Request {
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
        }.to_string(),
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
        ).to_string()
    );
}

#[test]
fn parser_loop() -> Result<(), rsip::Error> {
    let req: &str = concat!(
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
            );
    let parsed_request = Request::try_from(req.clone().as_bytes())?;
    assert_eq!(req, String::from(parsed_request).as_str());

    Ok(())
}

#[test]
fn parser() {
    assert_eq!(
        Request::try_from("REGISTER sip:server.com SIP/2.0\r\n\r\n".as_bytes()),
        Ok(Request {
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
        }),
    );

    assert_eq!(
        Request::try_from(
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
        Ok(Request {
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
        }),
    );
}

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize("REGISTER sip:server.com SIP/2.0\r\n\r\n".as_bytes()),
        Ok((
            "".as_bytes(),
            Tokenizer {
                method: "REGISTER".as_bytes().into(),
                uri: uri::Tokenizer {
                    schema: Some("sip".as_bytes().into()),
                    auth: None,
                    host_with_port: ("server.com".as_bytes(), None).into(),
                    params: vec![],
                    headers: None
                },
                version: ("2".as_bytes(), "0".as_bytes()).into(),
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
            "".as_bytes(),
            Tokenizer {
                method: "REGISTER".as_bytes().into(),
                uri: uri::Tokenizer {
                    schema: Some("sips".as_bytes().into()),
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
            }
        )),
    );

    assert!(Tokenizer::tokenize("SIP/2.0 401 Unauthorized\r\n\r\n".as_bytes()).is_err());
}
