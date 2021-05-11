use rsip::common::uri::{host_with_port::Tokenizer, Host, HostWithPort};
use std::convert::TryInto;

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
