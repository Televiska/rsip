pub fn generate_register() -> rsip::SipMessage {
    let mut headers: rsip::Headers = Default::default();

    let base_uri = rsip::Uri {
        scheme: Some(rsip::Scheme::Sips),
        auth: Some(("bob", Option::<String>::None).into()),
        host_with_port: rsip::Domain::from("biloxi.example.com").into(),
        ..Default::default()
    };

    headers.push(
        rsip::typed::Via {
            version: rsip::Version::V2,
            transport: rsip::Transport::Tls,
            uri: rsip::Uri {
                host_with_port: (rsip::Domain::from("client.biloxi.example.com"), 5060).into(),
                ..Default::default()
            },
            params: vec![rsip::Param::Branch(rsip::param::Branch::new(
                "z9hG4bKnashds7",
            ))],
        }
        .into(),
    );
    headers.push(rsip::headers::MaxForwards::default().into());
    headers.push(
        rsip::typed::From {
            display_name: Some("Bob".into()),
            uri: base_uri.clone(),
            params: vec![rsip::Param::Tag(rsip::param::Tag::new("a73kszlfl"))],
        }
        .into(),
    );
    headers.push(
        rsip::typed::To {
            display_name: Some("Bob".into()),
            uri: base_uri.clone(),
            params: Default::default(),
        }
        .into(),
    );
    headers.push(rsip::headers::CallId::default().into());
    headers.push(
        rsip::typed::CSeq {
            seq: 1,
            method: rsip::Method::Register,
        }
        .into(),
    );
    headers.push(
        rsip::typed::Contact {
            display_name: None,
            uri: base_uri,
            params: Default::default(),
        }
        .into(),
    );
    headers.push(rsip::headers::ContentLength::default().into());

    rsip::Request {
        method: rsip::Method::Register,
        uri: rsip::Uri {
            scheme: Some(rsip::Scheme::Sips),
            host_with_port: rsip::Domain::from("ss2.biloxi.example.com").into(),
            ..Default::default()
        },
        headers: headers,
        version: rsip::Version::V2,
        body: Default::default(),
    }
    .into()
}

pub fn create_unauthorized_from(request: rsip::Request) -> Result<rsip::SipMessage, rsip::Error> {
    use rsip::prelude::*;

    let mut headers: rsip::Headers = Default::default();
    headers.push(request.via_header()?.clone().into());
    headers.push(request.from_header()?.clone().into());
    let mut to = request.to_header()?.typed()?;
    to.with_tag("1410948204".into());
    headers.push(to.into());
    headers.push(request.call_id_header()?.clone().into());
    headers.push(request.cseq_header()?.clone().into());
    headers.push(rsip::Header::ContentLength(Default::default()));
    headers.push(rsip::Header::Server(Default::default()));

    headers.push(
        rsip::typed::WwwAuthenticate {
            realm: "atlanta.example.com".into(),
            nonce: "ea9c8e88df84f1cec4341ae6cbe5a359".into(),
            algorithm: Some(rsip::headers::auth::Algorithm::Md5),
            qop: Some(rsip::headers::auth::Qop::Auth),
            stale: Some("FALSE".into()),
            opaque: Some("".into()),
            ..Default::default()
        }
        .into(),
    );

    Ok(rsip::Response {
        status_code: 401.into(),
        headers,
        version: rsip::Version::V2,
        body: Default::default(),
    }
    .into())
}

fn main() -> Result<(), rsip::Error> {
    use std::convert::TryInto;

    let register_request = generate_register();
    println!("Sending REGISTER request: \n{}", register_request);

    let unauthorized_response = create_unauthorized_from(register_request.try_into()?)?;
    println!(
        "Responding with 401 to REGISTER request: \n{}",
        unauthorized_response
    );

    Ok(())
}
