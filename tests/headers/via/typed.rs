use rsip::{
    common::{
        uri::{self, param::Branch, HostWithPort, Param},
        Transport, Version,
    },
    headers::typed::{via::Tokenizer, Via},
};
use std::convert::TryInto;

validate_typed_header_trait!(Via);

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(
            format!(
                "{}",
                Via {
                    version: Version::V2,
                    transport: Transport::Tls,
                    uri: HostWithPort::from(("client.biloxi.example.com", Some(5061))).into(),
                    params: vec![Param::Branch(Branch::new("z9hG4bKnashds7"))]
                }
            ),
            String::from("SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashds7")
        )
    }
}

mod try_from_tokenizer {
    use super::*;

    #[test]
    fn try_from_1() {
        assert_eq!(
            Tokenizer {
                version: ("2".as_bytes(), "0".as_bytes()).into(),
                transport: "TLS".as_bytes().into(),
                uri: uri::Tokenizer {
                    scheme: None,
                    auth: None,
                    host_with_port: (
                        "client.biloxi.example.com".as_bytes(),
                        Some("5061".as_bytes())
                    )
                        .into(),
                    params: vec![],
                    headers: None
                },
                params: vec![("branch".as_bytes(), Some("z9hG4bKnashds7".as_bytes())).into()],
            }
            .try_into(),
            Ok(Via {
                version: Version::V2,
                transport: Transport::Tls,
                uri: HostWithPort::from(("client.biloxi.example.com", Some(5061))).into(),
                params: vec![Param::Branch(Branch::new("z9hG4bKnashds7"))]
            })
        );
    }
}
