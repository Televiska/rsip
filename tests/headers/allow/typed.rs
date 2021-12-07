use rsip::{
    common::Method,
    headers::typed::{allow::Tokenizer, Allow},
};
use std::convert::TryInto;

validate_typed_header_trait!(Allow);

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(
            Allow(vec![Method::Ack]).to_string(),
            String::from(concat!("ACK",))
        );
    }

    #[test]
    fn display2() {
        assert_eq!(
            Allow(vec![Method::Ack, Method::Bye]).to_string(),
            String::from(concat!("ACK, BYE",))
        );
    }
}

mod try_from_tokenizer {
    use super::*;

    #[test]
    fn try_from_1() -> Result<(), rsip::Error> {
        assert_eq!(
            Tokenizer {
                tokens: vec!["ACK"]
            }
            .try_into(),
            Ok(Allow(vec![Method::Ack]))
        );

        Ok(())
    }

    #[test]
    fn try_from_2() -> Result<(), rsip::Error> {
        assert_eq!(
            Tokenizer {
                tokens: vec!["ACK", "BYE"]
            }
            .try_into(),
            Ok(Allow(vec![Method::Ack, Method::Bye]))
        );

        Ok(())
    }

    #[test]
    fn try_from_3() -> Result<(), rsip::Error> {
        assert_eq!(
            Tokenizer {
                tokens: vec!["ACK", "BYE"]
            }
            .try_into(),
            Ok(Allow(vec![Method::Ack, Method::Bye]))
        );

        Ok(())
    }
}
