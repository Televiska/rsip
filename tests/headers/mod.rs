use rsip::headers::UntypedHeader;

pub mod accept;
pub mod alert_info;
pub mod allow;
pub mod auth;
pub mod authentication_info;
pub mod authorization;
pub mod call_info;
pub mod contact;
pub mod content_disposition;
pub mod content_length;
pub mod content_type;
pub mod cseq;
pub mod error_info;
pub mod from;
pub mod in_reply_to;
pub mod max_forwards;
pub mod media_type;
pub mod priority;
pub mod proxy_authenticate;
pub mod record_route;
pub mod reply_to;
pub mod to;
pub mod tokenizers;
pub mod via;
pub mod warning;
pub mod www_authenticate;

use rsip::headers::{header::Tokenizer, Accept, Header};

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(
            Header::Accept(Accept::new("REGISTER, INVITE")).to_string(),
            String::from("Accept: REGISTER, INVITE")
        );
    }

    #[test]
    fn display2() {
        assert_eq!(
            Header::Other("X-Forward".into(), "202.45.213.14".into()).to_string(),
            String::from("X-Forward: 202.45.213.14")
        );
    }
}

mod tokenizer {
    use super::*;

    #[test]
    fn tokenizer1() {
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
}
