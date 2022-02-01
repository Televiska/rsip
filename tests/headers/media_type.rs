use rsip::headers::typed::{media_type::Tokenizer, MediaType};
use std::convert::TryInto;

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(
            MediaType::Sdp(vec![]).to_string(),
            String::from("application/sdp")
        );
    }

    #[test]
    fn display2() {
        assert_eq!(
            MediaType::Sdp(vec![
                ("charset", "ISO-8859-4").into(),
                ("foo", "bar").into()
            ])
            .to_string(),
            String::from("application/sdp; charset=ISO-8859-4; foo=bar")
        );
    }

    #[test]
    fn display3() {
        assert_eq!(
            MediaType::Other("application/json".into(), vec![]).to_string(),
            String::from("application/json")
        );
    }

    #[test]
    fn display4() {
        assert_eq!(
            MediaType::Other(
                "application/json".into(),
                vec![("charset", "ISO-8859-4").into(), ("foo", "bar").into()]
            )
            .to_string(),
            String::from("application/json; charset=ISO-8859-4; foo=bar")
        );
    }
}

mod try_from_tokenizer {
    use super::*;

    #[test]
    fn try_from_1() {
        assert_eq!(
            Tokenizer {
                name: "application/sdp",
                params: vec![("charset", "ISO-8859-4")]
            }
            .try_into(),
            Ok(MediaType::Sdp(vec![("charset", "ISO-8859-4").into(),]))
        );
    }
}
