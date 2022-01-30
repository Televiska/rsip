use rsip::headers::typed::{tokenizers::MediaTypeTokenizer, Tokenize};

#[test]
fn tokenizer1() {
    assert_eq!(
        MediaTypeTokenizer::tokenize("application/sdp"),
        Ok(MediaTypeTokenizer {
            name: "application/sdp",
            params: vec![]
        })
    );
}

#[test]
fn tokenizer2() {
    assert_eq!(
        MediaTypeTokenizer::tokenize("application/sdp; charset=ISO-8859-4"),
        Ok(MediaTypeTokenizer {
            name: "application/sdp",
            params: vec![("charset", "ISO-8859-4")]
        })
    );
}

#[test]
fn tokenizer3() {
    assert_eq!(
        MediaTypeTokenizer::tokenize(
            "application/vnd.api+json; pagination=simple-spec; querying=graphql"
        ),
        Ok(MediaTypeTokenizer {
            name: "application/vnd.api+json",
            params: vec![("pagination", "simple-spec"), ("querying", "graphql")]
        })
    );
}
