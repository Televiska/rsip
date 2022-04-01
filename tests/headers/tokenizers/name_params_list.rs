use rsip::headers::typed::{
    tokenizers::{NameParamsListTokenizer, NameParamsTokenizer},
    Tokenize,
};

#[test]
fn tokenizer1() {
    assert_eq!(
        NameParamsListTokenizer::tokenize(
            "application/vnd.api+json; pagination=simple-spec; querying=graphql"
        ),
        Ok(NameParamsListTokenizer(vec![NameParamsTokenizer {
            name: "application/vnd.api+json",
            params: vec![("pagination", "simple-spec"), ("querying", "graphql")]
        }]))
    );
}

#[test]
fn tokenizer2() {
    assert_eq!(
        NameParamsListTokenizer::tokenize(concat!(
            "application/vnd.api+json; pagination=simple-spec; querying=graphql, ",
            "application/sdp; charset=ISO-8859-4"
        )),
        Ok(NameParamsListTokenizer(vec![
            NameParamsTokenizer {
                name: "application/vnd.api+json",
                params: vec![("pagination", "simple-spec"), ("querying", "graphql")]
            },
            NameParamsTokenizer {
                name: "application/sdp",
                params: vec![("charset", "ISO-8859-4")]
            }
        ]))
    );
}

#[test]
fn tokenizer3() {
    assert_eq!(
        NameParamsListTokenizer::tokenize(concat!(
            "application/vnd.api+json; pagination=simple-spec; querying=graphql,", //without space
            "application/sdp; charset=ISO-8859-4"
        )),
        Ok(NameParamsListTokenizer(vec![
            NameParamsTokenizer {
                name: "application/vnd.api+json",
                params: vec![("pagination", "simple-spec"), ("querying", "graphql")]
            },
            NameParamsTokenizer {
                name: "application/sdp",
                params: vec![("charset", "ISO-8859-4")]
            }
        ]))
    );
}

//taken from RFC3261
#[test]
fn tokenizer4() {
    assert_eq!(
        NameParamsListTokenizer::tokenize(concat!(
            "application/sdp;level=1, ",
            "application/x-private, ",
            "text/html"
        )),
        Ok(NameParamsListTokenizer(vec![
            NameParamsTokenizer {
                name: "application/sdp",
                params: vec![("level", "1")]
            },
            NameParamsTokenizer {
                name: "application/x-private",
                params: vec![]
            },
            NameParamsTokenizer {
                name: "text/html",
                params: vec![]
            }
        ]))
    );
}
