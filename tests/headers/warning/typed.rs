use rsip::common::Uri;
use rsip::headers::typed::{tokenizers::WarningTokenizer, Tokenize, Warning};
use std::convert::{TryFrom, TryInto};

validate_typed_header_trait!(Warning);

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(
            Warning {
                code: 307,
                uri: Uri::try_from("isi.edu").unwrap(),
                text: "Session parameter 'foo' not understood".into()
            }
            .to_string(),
            String::from("307 isi.edu \"Session parameter 'foo' not understood\"")
        );
    }

    #[test]
    fn display2() {
        assert_eq!(
            Warning {
                code: 301,
                uri: Uri::try_from("isi.edu").unwrap(),
                text: "\"Incompatible network address type 'E.164'\"".into()
            }
            .to_string(),
            String::from("301 isi.edu \"Incompatible network address type 'E.164'\"")
        );
    }
}

mod try_from_tokenizer {
    use super::*;

    #[test]
    fn try_from_1() -> Result<(), rsip::Error> {
        let tokenizer =
            WarningTokenizer::tokenize("307 isi.edu \"Session parameter 'foo' not understood\"")?;

        assert_eq!(
            tokenizer.try_into(),
            Ok(Warning {
                code: 307,
                uri: Uri::try_from("isi.edu").unwrap(),
                text: "Session parameter 'foo' not understood".into()
            })
        );

        Ok(())
    }

    #[test]
    fn try_from_2() -> Result<(), rsip::Error> {
        let tokenizer =
            WarningTokenizer::tokenize("301 isi.edu Incompatible network address type 'E.164'")?;

        assert_eq!(
            tokenizer.try_into(),
            Ok(Warning {
                code: 301,
                uri: Uri::try_from("isi.edu").unwrap(),
                text: "Incompatible network address type 'E.164'".into()
            })
        );

        Ok(())
    }
}
