use rsip::headers::typed::{tokenizers::NameParamsTokenizer, Tokenize};
/*
 * TODO: fix me, this is green!
#[test]
fn tokenizer0() {
    assert_eq!(
        NameParamsTokenizer::tokenize(""),
        Ok(NameParamsTokenizer {
            name: "",
            params: vec![]
        })
    );
}
*/

#[test]
fn tokenizer1() {
    assert_eq!(
        NameParamsTokenizer::tokenize("application/sdp"),
        Ok(NameParamsTokenizer {
            name: "application/sdp",
            params: vec![]
        })
    );
}

#[test]
fn tokenizer2() {
    assert_eq!(
        NameParamsTokenizer::tokenize("application/sdp; charset=ISO-8859-4"),
        Ok(NameParamsTokenizer {
            name: "application/sdp",
            params: vec![("charset", "ISO-8859-4")]
        })
    );
}

#[test]
fn tokenizer3() {
    assert_eq!(
        NameParamsTokenizer::tokenize(
            "application/vnd.api+json; pagination=simple-spec; querying=graphql"
        ),
        Ok(NameParamsTokenizer {
            name: "application/vnd.api+json",
            params: vec![("pagination", "simple-spec"), ("querying", "graphql")]
        })
    );
}

#[test]
fn tokenizer4() {
    assert_eq!(
        NameParamsTokenizer::tokenize("attachment; filename=smime.p7s; handling=required"),
        Ok(NameParamsTokenizer {
            name: "attachment",
            params: vec![("filename", "smime.p7s"), ("handling", "required")]
        })
    );
}
