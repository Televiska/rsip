use rsip::common::uri::auth::{Auth, Tokenizer};
use std::convert::TryInto;

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"user:password@server2.com SIP/2.0"),
        Ok((
            "server2.com SIP/2.0".as_bytes(),
            ("user".as_bytes(), Some("password".as_bytes())).into()
        )),
    );

    assert!(Tokenizer::tokenize(b"server2.com SIP/2.0").is_err());
}

#[test]
fn parser() {
    assert_eq!(
        Tokenizer::from(("user".as_bytes(), Some("password".as_bytes()))).try_into(),
        Ok(Auth {
            username: "user".into(),
            password: Some("password".into())
        }),
    );
}
