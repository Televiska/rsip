pub mod auth;
pub mod host_with_port;
pub mod params;
pub mod schema;

use rsip::common::uri::{Auth, Host, HostWithPort, Schema, Tokenizer, Uri};

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"server2.com SIP/2.0"),
        Ok((
            b"SIP/2.0".as_ref(),
            Tokenizer {
                schema: None,
                auth: None,
                host_with_port: (b"server2.com".as_ref(), None).into(),
                params: None,
                headers: None
            }
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b"user@server2.com SIP/2.0"),
        Ok((
            b"SIP/2.0".as_ref(),
            Tokenizer {
                schema: None,
                auth: Some((b"user".as_ref(), None).into()),
                host_with_port: (b"server2.com".as_ref(), None).into(),
                params: None,
                headers: None
            }
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b"user:password@server2.com SIP/2.0"),
        Ok((
            b"SIP/2.0".as_ref(),
            Tokenizer {
                schema: None,
                auth: Some((b"user".as_ref(), Some(b"password".as_ref())).into()),
                host_with_port: (b"server2.com".as_ref(), None).into(),
                params: None,
                headers: None
            }
        )),
    );

    assert_eq!(
        Tokenizer::tokenize(b"user:password@server2.com:5060 SIP/2.0"),
        Ok((
            b"SIP/2.0".as_ref(),
            Tokenizer {
                schema: None,
                auth: Some((b"user".as_ref(), Some(b"password".as_ref())).into()),
                host_with_port: (b"server2.com".as_ref(), Some(b"5060".as_ref())).into(),
                params: None,
                headers: None
            }
        )),
    );
}
