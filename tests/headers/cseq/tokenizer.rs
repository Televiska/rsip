use rsip::headers::typed::{cseq::Tokenizer, Tokenize};

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
