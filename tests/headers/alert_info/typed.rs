use rsip::{
    common::uri::UriWithParamsList,
    headers::typed::{tokenizers::UriWithParamsListTokenizer, AlertInfo},
};
use std::convert::TryInto;
use testing_utils::Randomize;

validate_typed_header_trait!(AlertInfo);

mod display {
    use super::*;

    #[test]
    fn display1() {
        let uri_with_params_list = UriWithParamsList::random();
        assert_eq!(
            AlertInfo(uri_with_params_list.clone()).to_string(),
            uri_with_params_list.to_string()
        );
    }
}

mod try_from_tokenizer {
    use super::*;

    #[test]
    fn try_from_1() -> Result<(), rsip::Error> {
        let uri_with_params_list = UriWithParamsList::random();
        let uri_with_params_list_raw = uri_with_params_list.to_string();
        let tokenizer = UriWithParamsListTokenizer::tokenize(&uri_with_params_list_raw)
            .unwrap()
            .1;

        assert_eq!(tokenizer.try_into(), Ok(AlertInfo(uri_with_params_list)));

        Ok(())
    }
}
