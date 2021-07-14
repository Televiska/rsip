use rsip::{headers::*, message::HeadersExt};

#[test]
fn headers() {
    //TODO: start using randomized stuff/traits
    let via = Via::new("SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashd92");
    let max_forwards = MaxForwards::new("70");
    let to = To::new("Bob <sips:bob@biloxi.example.com>");
    let from = From::new("Bob <sips:bob@biloxi.example.com>;tag=ja743ks76zlflH");
    let call_id = CallId::new("1j9FpLxk3uxtm8tn@biloxi.example.com");
    let cseq = CSeq::new("2 REGISTER");
    let contact = Contact::new("<sips:bob@client.biloxi.example.com>");
    let authorization = Authorization::new("Digest username=\"bob\", realm=\"atlanta.example.com\" nonce=\"ea9c8e88df84f1cec4341ae6cbe5a359\", opaque=\"\" uri=\"sips:ss2.biloxi.example.com\", response=\"dfe56131d1958046689d83306477ecc\"");
    let content_length = ContentLength::new("0");

    let headers: rsip::headers::Headers = vec![
        via.clone().into(),
        max_forwards.clone().into(),
        from.clone().into(),
        to.clone().into(),
        call_id.clone().into(),
        cseq.clone().into(),
        contact.clone().into(),
        authorization.clone().into(),
        content_length.clone().into(),
    ]
    .into();

    let implementer = crate::support::HasHeadersImpl(headers);

    assert_eq!(implementer.via_header(), Ok(&via));
    assert_eq!(implementer.max_forwards_header(), Ok(&max_forwards));
    assert_eq!(implementer.to_header(), Ok(&to));
    assert_eq!(implementer.from_header(), Ok(&from));
    assert_eq!(implementer.call_id_header(), Ok(&call_id));
    assert_eq!(implementer.cseq_header(), Ok(&cseq));
    assert_eq!(implementer.contact_header(), Ok(&contact));
    assert_eq!(implementer.authorization_header(), Some(&authorization));
}
