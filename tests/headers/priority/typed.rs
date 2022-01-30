use rsip::headers::typed::{tokenizers::ValueTokenizer, Priority, Tokenize};
use std::convert::TryInto;
//use testing_utils::Randomize;

validate_typed_header_trait!(Priority);

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(Priority::NonUrgent.to_string(), "non-urgent");
    }

    #[test]
    fn display2() {
        assert_eq!(Priority::Other("FooBar".into()).to_string(), "FooBar");
    }
}

mod try_from_tokenizer {
    use super::*;

    #[test]
    fn try_from_1() -> Result<(), rsip::Error> {
        let tokenizer = ValueTokenizer::tokenize("emergency").unwrap();

        assert_eq!(tokenizer.try_into(), Ok(Priority::Emergency));

        Ok(())
    }

    #[test]
    fn try_from_2() -> Result<(), rsip::Error> {
        let tokenizer = ValueTokenizer::tokenize("FooBar").unwrap();

        assert_eq!(tokenizer.try_into(), Ok(Priority::Other("FooBar".into())));

        Ok(())
    }
}
