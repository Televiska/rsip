use rsip::common::uri::auth::{Auth, Tokenizer};

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"user:password@server2.com SIP/2.0"),
        Ok((
            b"server2.com SIP/2.0".as_ref(),
            (b"user".as_ref(), Some(b"password".as_ref())).into()
        )),
    );

    assert!(Tokenizer::tokenize(b"server2.com SIP/2.0").is_err());
}

#[test]
fn parser() {
    assert_eq!(
        Auth::parse((b"user".as_ref(), Some(b"password".as_ref())).into()),
        Ok(Auth {
            username: "user".into(),
            password: Some("password".into())
        }),
    );
}
