use rsip::headers::typed::{tokenizers::ValueTokenizer, Tokenize};

#[test]
fn tokenizer1() {
    assert_eq!(
        ValueTokenizer::tokenize("testrealm@host.com"),
        Ok(ValueTokenizer {
            value: "testrealm@host.com"
        })
    );
}
