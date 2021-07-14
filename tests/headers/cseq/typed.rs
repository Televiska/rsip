use rsip::{
    common::Method,
    headers::typed::{cseq::Tokenizer, CSeq},
};
use std::convert::TryInto;

validate_typed_header_trait!(CSeq);

#[test]
fn from_tokenizer() {
    assert_eq!(
        Tokenizer {
            seq: "123",
            method: "INVITE"
        }
        .try_into(),
        Ok(CSeq {
            seq: 123,
            method: Method::Invite
        })
    );
}
