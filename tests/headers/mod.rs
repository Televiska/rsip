use rsip::headers::UntypedHeader;

pub mod accept;
pub mod auth;
pub mod authorization;
pub mod contact;
pub mod content_length;
pub mod cseq;
pub mod from;
pub mod max_forwards;
pub mod to;
pub mod via;
pub mod www_authenticate;

use rsip::headers::{header::Tokenizer, Accept, Header};

#[test]
fn display() {
    assert_eq!(
        Header::Accept(Accept::new("REGISTER, INVITE")).to_string(),
        String::from("Accept: REGISTER, INVITE")
    );

    assert_eq!(
        Header::Other("X-Forward".into(), "202.45.213.14".into()).to_string(),
        String::from("X-Forward: 202.45.213.14")
    );
}

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"Accept: REGISTER, INVITE\r\n something"),
        Ok((
            " something".as_bytes(),
            Tokenizer {
                name: "Accept".as_bytes(),
                value: "REGISTER, INVITE".as_bytes()
            }
        )),
    );
}
