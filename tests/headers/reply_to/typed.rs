use rsip::{
    common::uri::UriWithParams,
    headers::typed::{tokenizers::UriWithParamsTokenizer, ReplyTo},
};
use std::convert::TryInto;
use testing_utils::Randomize;

validate_typed_header_trait!(ReplyTo);

mod display {
    use super::*;

    #[test]
    fn display1() {
        let uri_with_params = UriWithParams::random();
        assert_eq!(
            ReplyTo(uri_with_params.clone()).to_string(),
            uri_with_params.to_string()
        );
    }
}

mod try_from_tokenizer {
    use super::*;

    #[test]
    fn try_from_1() -> Result<(), rsip::Error> {
        let uri_with_params = UriWithParams::random();
        let uri_with_params_raw = uri_with_params.to_string();
        let tokenizer = UriWithParamsTokenizer::tokenize(&uri_with_params_raw)
            .unwrap()
            .1;

        assert_eq!(tokenizer.try_into(), Ok(ReplyTo(uri_with_params)));

        Ok(())
    }
}
