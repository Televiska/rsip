use rsip::headers::typed::{accept::Tokenizer, tokenizers::NameParamsTokenizer, Accept, MediaType};
use std::convert::TryInto;

validate_typed_header_trait!(Accept);

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(
            Accept(vec![MediaType::Sdp(vec![])]).to_string(),
            String::from(concat!("application/sdp",))
        );
    }

    #[test]
    fn display2() {
        assert_eq!(
            Accept(vec![
                MediaType::Sdp(vec![
                    ("charset", "ISO-8859-4").into(),
                    ("foo", "bar").into()
                ]),
                MediaType::Other("application/json".into(), vec![("version", "v2").into()])
            ])
            .to_string(),
            String::from(concat!(
                "application/sdp; charset=ISO-8859-4; foo=bar, application/json; version=v2"
            ))
        );
    }
}

mod try_from_tokenizer {
    use super::*;

    #[test]
    fn try_from_1() -> Result<(), rsip::Error> {
        assert_eq!(
            Tokenizer(vec![
                NameParamsTokenizer {
                    name: "application/sdp".into(),
                    params: vec![("charset", "ISO-8859-4").into(), ("foo", "bar").into()]
                },
                NameParamsTokenizer {
                    name: "application/json".into(),
                    params: vec![("version", "v2").into()]
                },
            ])
            .try_into(),
            Ok(Accept(vec![
                MediaType::Sdp(vec![
                    ("charset", "ISO-8859-4").into(),
                    ("foo", "bar").into()
                ]),
                MediaType::Other("application/json".into(), vec![("version", "v2").into()])
            ]))
        );

        Ok(())
    }
}
