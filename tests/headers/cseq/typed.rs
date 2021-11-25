use rsip::{
    common::Method,
    headers::typed::{cseq::Tokenizer, CSeq},
};
use std::convert::TryInto;

validate_typed_header_trait!(CSeq);

mod try_from_tokenizer {
    use super::*;

    #[test]
    fn try_from1() {
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
}
