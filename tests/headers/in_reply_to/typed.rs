use rsip::headers::typed::{tokenizers::TokenListTokenizer, InReplyTo, Tokenize};
use std::convert::TryInto;
//use testing_utils::Randomize;

validate_typed_header_trait!(InReplyTo);

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(
            InReplyTo(vec![
                "70710@saturn.bell-tel.com".into(),
                "17320@saturn.bell-tel.com".into()
            ])
            .to_string(),
            "70710@saturn.bell-tel.com, 17320@saturn.bell-tel.com".to_string()
        );
    }
}

mod try_from_tokenizer {
    use super::*;

    #[test]
    fn try_from_1() -> Result<(), rsip::Error> {
        let tokenizer =
            TokenListTokenizer::tokenize(&"70710@saturn.bell-tel.com, 17320@saturn.bell-tel.com")
                .unwrap();

        assert_eq!(
            tokenizer.try_into(),
            Ok(InReplyTo(vec![
                "70710@saturn.bell-tel.com".into(),
                "17320@saturn.bell-tel.com".into()
            ]))
        );

        Ok(())
    }
}
