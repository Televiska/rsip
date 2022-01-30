use rsip::headers::typed::{tokenizers::WarningTokenizer, Tokenize};

#[test]
fn tokenizer1() {
    assert_eq!(
        WarningTokenizer::tokenize("301 isi.edu \"Incompatible network address type 'E.164'\""),
        Ok(WarningTokenizer {
            code: "301",
            host: "isi.edu",
            text: "Incompatible network address type 'E.164'",
        })
    );
}

#[test]
fn tokenizer2() {
    assert_eq!(
        WarningTokenizer::tokenize("307 isi.edu Session parameter 'foo' not understood"),
        Ok(WarningTokenizer {
            code: "307",
            host: "isi.edu",
            text: "Session parameter 'foo' not understood",
        })
    );
}
