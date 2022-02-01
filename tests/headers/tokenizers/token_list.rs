use rsip::headers::typed::{tokenizers::TokenListTokenizer, Tokenize};

#[test]
fn tokenizer1() {
    assert_eq!(
        TokenListTokenizer::tokenize("ACK"),
        Ok(TokenListTokenizer {
            tokens: vec!["ACK"],
        })
    );
}

#[test]
fn tokenizer2() {
    assert_eq!(
        TokenListTokenizer::tokenize(" ACK "),
        Ok(TokenListTokenizer {
            tokens: vec!["ACK"],
        })
    );
}

#[test]
fn tokenizer3() {
    assert_eq!(
        TokenListTokenizer::tokenize("foo, ACK "),
        Ok(TokenListTokenizer {
            tokens: vec!["foo", "ACK"],
        })
    );
}

#[test]
fn tokenizer4() {
    assert_eq!(
        TokenListTokenizer::tokenize(" foo , ACK "),
        Ok(TokenListTokenizer {
            tokens: vec!["foo", "ACK"],
        })
    );
}
