use rsip::headers::typed::{
    content_disposition::{DisplayType, Tokenizer},
    ContentDisposition,
};
use std::convert::TryInto;

validate_typed_header_trait!(ContentDisposition);

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(
            ContentDisposition {
                display_type: DisplayType::Session,
                display_params: vec![]
            }
            .to_string(),
            String::from(concat!("session",))
        );
    }

    #[test]
    fn display2() {
        assert_eq!(
            ContentDisposition {
                display_type: DisplayType::Other("attachment".into()),
                display_params: vec![
                    ("filename", "smime.p7s").into(),
                    ("handling", "required").into()
                ]
            }
            .to_string(),
            String::from(concat!("attachment; filename=smime.p7s; handling=required"))
        );
    }
}

mod try_from_tokenizer {
    use super::*;

    #[test]
    fn try_from_1() -> Result<(), rsip::Error> {
        assert_eq!(
            Tokenizer {
                name: "attachment",
                params: vec![("filename", "smime.p7s"), ("handling", "required")]
            }
            .try_into(),
            Ok(ContentDisposition {
                display_type: DisplayType::Other("attachment".into()),
                display_params: vec![
                    ("filename", "smime.p7s").into(),
                    ("handling", "required").into()
                ]
            })
        );

        Ok(())
    }
}
