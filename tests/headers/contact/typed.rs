use rsip::{
    common::uri,
    headers::typed::{contact::Tokenizer, Contact},
};
use std::convert::{TryFrom, TryInto};

validate_typed_header_trait!(Contact);

mod display {
    use super::*;

    #[test]
    fn display1() -> Result<(), rsip::Error> {
        assert_eq!(
            format!(
                "{}",
                Contact {
                    display_name: Some("Alice".into()),
                    uri: uri::Uri {
                        scheme: Some(uri::Scheme::Sip),
                        auth: Some(uri::Auth {
                            user: "alice".into(),
                            password: None
                        }),
                        host_with_port: uri::HostWithPort::try_from("atlanta.example.com")?,
                        params: vec![],
                        headers: vec![].into()
                    },
                    params: vec![uri::Param::Tag(uri::param::Tag::new("9fxced76sl"))]
                }
            ),
            String::from("Alice <sip:alice@atlanta.example.com>;tag=9fxced76sl")
        );

        Ok(())
    }
}

mod try_from_tokenizer {
    use super::*;

    #[test]
    fn try_from_1() -> Result<(), rsip::Error> {
        assert_eq!(
            Tokenizer {
                display_name: Some("Alice"),
                uri: uri::Tokenizer {
                    scheme: Some("sip".into()),
                    auth: Some(uri::auth::Tokenizer::from(("alice", None,))),
                    host_with_port: ("atlanta.example.com", None).into(),
                    params: vec![],
                    headers: None,
                    ..Default::default()
                },
                params: vec![("tag", Some("9fxced76sl")).into()],
            }
            .try_into(),
            Ok(Contact {
                display_name: Some("Alice".into()),
                uri: uri::Uri {
                    scheme: Some(uri::Scheme::Sip),
                    auth: Some(uri::Auth {
                        user: "alice".into(),
                        password: None
                    }),
                    host_with_port: uri::HostWithPort::try_from("atlanta.example.com")?,
                    params: vec![],
                    headers: vec![].into()
                },
                params: vec![uri::Param::Tag(uri::param::Tag::new("9fxced76sl"))]
            })
        );

        Ok(())
    }
}
