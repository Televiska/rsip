use rsip::common::uri::{host_with_port::Tokenizer, Host, HostWithPort};

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"server2.com SIP/2.0"),
        Ok((b"SIP/2.0".as_ref(), (b"server2.com".as_ref(), None).into())),
    );

    assert_eq!(
        Tokenizer::tokenize(b"server2.com:5060 SIP/2.0"),
        Ok((
            b"SIP/2.0".as_ref(),
            (b"server2.com".as_ref(), Some(b"5060".as_ref())).into()
        )),
    );
}

#[test]
fn parser() {
    assert_eq!(
        HostWithPort::parse((b"server2.com".as_ref(), None).into()),
        Ok(HostWithPort {
            host: Host::Domain("server2.com".into()),
            port: None
        })
    );

    assert_eq!(
        HostWithPort::parse((b"server2.com".as_ref(), Some(b"5060".as_ref())).into()),
        Ok(HostWithPort {
            host: Host::Domain("server2.com".into()),
            port: Some(5060.into())
        })
    );
}
