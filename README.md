# Rsip

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Build status](https://github.com/vasilakisfil/rsip/actions/workflows/rust-ci.yml/badge.svg)
[![Crates.io Version](https://img.shields.io/crates/v/rsip.svg)](https://crates.io/crates/rsip)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.44.0+-lightgray.svg)](#rust-version-requirements)

A common general purpose library for SIP. It can parse and generate all SIP
structures.

Like [HTTP](https://github.com/hyperium/http), this crate is a general purpose library for common types found when
working with the SIP protocol.
You’ll find the `SipMessage` and its `Request` and `Response` variant types for working
as either a client or a server as well as all of their components, like `Method`,
`Version`, a very flexible `Uri`, `StatusCode` etc.

Rsip is capable of parsing messages from bytes, &str or String using [nom](https://github.com/Geal/nom)
parser and can also generate SIP messages using helpful structs.

You will notably not find an implementation of sending requests or spinning up a
SIP server in this crate. SIP servers, by nature of SIP protocol, are very complex
usually and will sit at different crates/libs. Rsip is intended to be the de-facto
SIP base library for Rust. It was built to be used inside [viska](https://github.com/vasilakisfil/viska)
initially but then was split to a different crate.

It was inspired by [libsip](https://github.com/ByteHeathen/libsip) but has taken
a bit different path regarding parsing, flexibility & safety.


For locating SIP servers ([RFC3263](https://datatracker.ietf.org/doc/html/rfc3263)) take a look on [rsip-dns](https://github.com/vasilakisfil/rsip-dns) library.

## Features
* This thing is _fast_, uses nom for basic message parsing and headers are parsed
  only when needed, on-demand. Intentions are to make it even faster by providing
  non-owning variants (`&str` and `&[u8]`)
* Strong (new)types everywhere. Even if underlying type is String, everything is
  a NewType for better type safety.
* Provides typed headers on demand, like `From`, `To`, `Contact`, `Via` etc
  The reasoning behind on demand strongly typed headers is 2 fold:
  * perfromance & memory reasons: headers are parsed only when needed
  * it enables you to still have a working Rust SIP parser in case a typed header
  has a bug, the peer has a bug or there is an edge/new case never seen before.
* While performance is always a goal, user friendliness and usability is the main
 goal. A lot of helpful functions and convertions to make things easy :)
* Very simple code structure make it super easy to extend and add new typed headers
  As long as you can do [nom](https://github.com/Geal/nom) stuff, it's straightforward. The goal is to add
  many typed headers of latest RFCs like [PASSporT](https://datatracker.ietf.org/doc/html/rfc8224), [SHAKEN](https://datatracker.ietf.org/doc/html/rfc8588), [push notifications](https://datatracker.ietf.org/doc/html/rfc8599) etc
* Provides some extra services like Digest auth generator/validator etc
  Intention is to add many helper services.

## Architecture
Each type in rsip has a tokenizer attached.
This is not enforced by the type system yet, however very soon this will be the case.
In brief, for every rsip type we have: 
* Tokenizing: in the lowest level we have the `Tokenizer` which is capable of tokenizing the input.
All common tokenizers accept abstract input, either `&str` or `&[u8]` so it can be reused when
the input is plain bytes, or when the input has already been parsed and it's a `String`/`&str`,
like the headers.
* Parsing: once the input has been tokenized, then there are `TryFrom` impls from the relevant type
tokenizer to the actual type.
This is the parsing step where tokens (in the form of `&str` or `&[u8]`) are transformed to
integers, strings and rsip types.
* each rsip type implements the `Display` trait and hence has a representation.

## Examples
For instance, generating the Register request found in [section 2.1 of RFC3665](https://datatracker.ietf.org/doc/html/rfc3665#section-2.1)

```
REGISTER sips:ss2.biloxi.example.com SIP/2.0
Via: SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashds7
Max-Forwards: 70
From: Bob <sips:bob@biloxi.example.com>;tag=a73kszlfl
To: Bob <sips:bob@biloxi.example.com>
Call-ID: 1j9FpLxk3uxtm8tn@biloxi.example.com
CSeq: 1 REGISTER
Contact: <sips:bob@client.biloxi.example.com>
Content-Length: 0
```

can be done like that:

```rust
fn generate_register_request() -> rsip::SipMessage {
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
        version: rsip::Version::V2,
        headers: headers,
        body: Default::default(),
    }
    .into()
}
```

And the response similarly can be generated:

```rust
pub fn create_unauthorized_from(request: rsip::Request) -> Result<rsip::SipMessage, crate::Error> {
    //imports helpful header traits
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
        body: Default::default()
    }
    .into())
}
```

which generates the following:

```
SIP/2.0 401 Unauthorized
Via: SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashds7
 ;received=192.0.2.201
From: Bob <sips:bob@biloxi.example.com>;tag=a73kszlfl
To: Bob <sips:bob@biloxi.example.com>;tag=1410948204
Call-ID: 1j9FpLxk3uxtm8tn@biloxi.example.com
CSeq: 1 REGISTER
WWW-Authenticate: Digest realm="atlanta.example.com", qop="auth",
 nonce="ea9c8e88df84f1cec4341ae6cbe5a359",
 opaque="", stale=FALSE, algorithm=MD5
Content-Length: 0
```

## To Do
* improve errors
* write more tests, especially around edge cases
* Make tokenizer an associated generic type on each type defined in this lib
* implement more common traits like Hash etc
