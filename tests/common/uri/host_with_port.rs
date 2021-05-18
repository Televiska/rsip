use rsip::common::uri::{host_with_port::Tokenizer, Host, HostWithPort};
use std::convert::TryInto;

#[test]
fn display() {
    assert_eq!(
        HostWithPort {
            host: Host::Domain("server2.com".into()),
            port: None
        }.to_string(),
        String::from("server2.com")
    );

    assert_eq!(
        HostWithPort {
            host: Host::Domain("server2.com".into()),
            port: Some(5060.into())
        }.to_string(),
        String::from("server2.com:5060")
    );
}

#[test]
fn parser() {
    assert_eq!(
        Tokenizer::from(("server2.com".as_bytes(), None)).try_into(),
        Ok(HostWithPort {
            host: Host::Domain("server2.com".into()),
            port: None
        })
    );

    assert_eq!(
        Tokenizer::from(("server2.com".as_bytes(), Some("5060".as_bytes()))).try_into(),
        Ok(HostWithPort {
            host: Host::Domain("server2.com".into()),
            port: Some(5060.into())
        })
    );
}

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"server2.com something"),
        Ok((
            " something".as_bytes(),
            ("server2.com".as_bytes(), None).into()
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b"server2.com:5060 something"),
        Ok((
            " something".as_bytes(),
            ("server2.com".as_bytes(), Some("5060".as_bytes())).into()
        )),
    );
}
