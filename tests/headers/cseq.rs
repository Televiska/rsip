use rsip::headers::header::Tokenize;
use rsip::{
    common::Method,
    headers::header::cseq::{self, Tokenizer},
};
use std::convert::TryInto;

validate_untyped_header_trait!(cseq, CSeq);

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenize::tokenize("123  INVITE"),
        Ok(Tokenizer {
            seq: "123",
            method: "INVITE"
        })
    );
}

mod typed {
    use super::*;

    validate_untyped_header_trait!(cseq, CSeq);

    #[test]
    fn from_tokenizer() {
        assert_eq!(
            Tokenizer {
                seq: "123",
                method: "INVITE"
            }
            .try_into(),
            Ok(cseq::typed::CSeq {
                seq: 123,
                method: Method::Invite
            })
        );
    }
}
