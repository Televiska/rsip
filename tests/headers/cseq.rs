use rsip::{
    common::Method,
    headers::header::cseq::{typed, Tokenizer},
};
use std::convert::TryInto;

//TODO: add automated tests for conversions
#[test]
fn typed() {
    assert_eq!(
        Tokenizer {
            seq: "123",
            method: "INVITE"
        }
        .try_into(),
        Ok(typed::CSeq {
            seq: 123,
            method: Method::Invite
        })
    );
}

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize("123  INVITE"),
        Ok(Tokenizer {
            seq: "123",
            method: "INVITE"
        })
    );
}
