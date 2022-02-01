use rsip::{
    common::uri::{Param, Scheme, Uri, UriWithParams, UriWithParamsList},
    headers::typed::{tokenizers::UriWithParamsListTokenizer, Route},
};
use std::convert::TryInto;
use testing_utils::Randomize;

validate_typed_header_trait!(Route);

mod display {
    use super::*;

    #[test]
    fn display1() {
        let uri_with_params_list = UriWithParamsList::random();
        assert_eq!(
            Route(uri_with_params_list.clone()).to_string(),
            uri_with_params_list.to_string()
        );
    }

    #[test]
    fn display2() {
        let uri_with_params_list = vec![
            UriWithParams {
                uri: Uri {
                    scheme: Some(Scheme::Sip),
                    host_with_port: "server10.biloxi.com".try_into().unwrap(),
                    params: vec![Param::Lr],
                    ..Default::default()
                },
                ..Default::default()
            },
            UriWithParams {
                uri: Uri {
                    scheme: Some(Scheme::Sip),
                    host_with_port: "bigbox3.site3.atlanta.com".try_into().unwrap(),
                    params: vec![Param::Lr],
                    ..Default::default()
                },
                ..Default::default()
            },
        ]
        .into();

        assert_eq!(
            Route(uri_with_params_list).to_string(),
            "<sip:server10.biloxi.com;lr>,<sip:bigbox3.site3.atlanta.com;lr>"
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

        assert_eq!(tokenizer.try_into(), Ok(Route(uri_with_params_list)));

        Ok(())
    }

    #[test]
    fn try_from_2() -> Result<(), rsip::Error> {
        let uris = "<sip:server10.biloxi.com;lr>,<sip:bigbox3.site3.atlanta.com;lr>";
        let uri_with_params_list = vec![
            UriWithParams {
                uri: Uri {
                    scheme: Some(Scheme::Sip),
                    host_with_port: "server10.biloxi.com".try_into().unwrap(),
                    params: vec![Param::Lr],
                    ..Default::default()
                },
                ..Default::default()
            },
            UriWithParams {
                uri: Uri {
                    scheme: Some(Scheme::Sip),
                    host_with_port: "bigbox3.site3.atlanta.com".try_into().unwrap(),
                    params: vec![Param::Lr],
                    ..Default::default()
                },
                ..Default::default()
            },
        ]
        .into();

        let tokenizer = UriWithParamsListTokenizer::tokenize(&uris).unwrap().1;

        assert_eq!(tokenizer.try_into(), Ok(Route(uri_with_params_list)));

        Ok(())
    }
}

