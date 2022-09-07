//! A general purpose library of common SIP types
//!
//! Like [http](https://docs.rs/http/) crate, this crate is a general purpose
//! library for common types found when working
//! with the SIP protocol. You'll find [SipMessage](SipMessage)
//! and its [Request](Request)
//! / [Response](Response)
//! variant types for working as either a client or a server as well
//! as all of their components, like [Method](Method),
//! a very flexible [Uri](Uri),
//! [Version](Version),
//! [StatusCode](StatusCode) etc.
//!
//! Rsip is capable of parsing messages from `bytes`, `&str` or `String` using
//! [nom](https://github.com/Geal/nom) parser and can also generate SIP messages
//! using helpful struct builders.
//!
//! You will notably not find an implementation of sending requests or spinning up
//! a SIP server in this crate. SIP servers, by nature of SIP protocol, are very
//! complex usually and will sit at different crates/libs. Rsip is intended to be
//! the de-facto SIP base library for Rust. It was built to be used inside
//! [viska](https://github.com/vasilakisfil/viska) initially but then was split
//! to a different crate. It was inspired by [libsip](https://github.com/ByteHeathen/libsip)
//! but has taken a bit different path regarding parsing, flexibility & safety.
//!
//! For locating SIP servers ([RFC3263](https://datatracker.ietf.org/doc/html/rfc3263)) take a
//! look on [rsip-dns](https://github.com/vasilakisfil/rsip-dns) library.
//!
//! ## SIP: It's all about headers
//! In case you haven't worked with SIP before let me tell you one thing: headers in SIP play
//! a crucial role, basically most of your logic resides around the headers (and there are plenty
//! of them :) ). `Status` and `Method` of course have their role, but there decisions are more
//! straightforward on what you need to do.
//!
//! Rsip takes a unique role regarding headers mixing together safety and flexibility: by default
//! rsip will parse all headers and convert them to distinct
//! [untyped header](headers::untyped) types,
//! as part of the big fat [Header enum](Header).
//! Each untyped header is basically a NewType around String.
//! This means that, apart from non-UTF8 compliant headers, any header is supported even if your
//! client has a bug or rsip itself has a bug somewhere in a typed header.
//! Also, any parsing on complex headers takes place only when needed, on demand.
//!
//! [Untyped headers](headers::untyped) have their use, but in practice when you want to
//! interact with a header, you need to convert it to its [typed form](headers::typed).
//! Not all headers have a typed form, for instance, some
//! headers are just opaque strings (like [CallID](headers::CallId)), or some headers just need helpful methods
//! instead of a whole new typed struct (like
//! [Expires](headers::Expires), which provides the [seconds](headers::Expires::seconds) method).
//!
//! For instance, creating a new `Call-ID` header can easily be done like (examples taken from
//! [RFC3665](https://datatracker.ietf.org/doc/html/rfc3665)):
//!
//! ```
//! use rsip::headers::UntypedHeader;
//! rsip::headers::CallId::new("1j9FpLxk3uxtm8tn@biloxi.example.com");
//! ```
//!
//! Similarly, generating a `From` header:
//! ```
//! use rsip::headers::{From, UntypedHeader};
//! let from = rsip::headers::From::new("Bob <sips:bob@biloxi.example.com>;tag=a73kszlfl");
//! ```
//!
//! If you want to automatically include all defined traits in Rsip, you can include the prelude:
//! ```
//! use rsip::prelude::*;
//! let from = rsip::headers::From::new("Bob <sips:bob@biloxi.example.com>;tag=a73kszlfl");
//! ```
//!
//! While for `Call-Id`, it's fine to pass in an str (which probably will be generated & saved
//! somewhere else in your application), for many headers working with plain strings doesn't make
//! much sense, like in the `From` header. As mentioned, the main reason untyped headers exist is
//! performance (parsing takes place only when needed) and flexibility (you can throw in there
//! whatever you like ^_^).
//!
//! In order to generate a From header, you will probably need to use its [typed](typed::From) version:
//! ```
//! use rsip::headers::typed::TypedHeader;
//!
//! let typed_from: rsip::typed::From = rsip::typed::From {
//!     display_name: Some("Bob".into()),
//!     uri: rsip::Uri {
//!         scheme: Some(rsip::Scheme::Sips),
//!         auth: Some(rsip::Auth {
//!             user: "Bob".into(),
//!             password: None,
//!         }),
//!         host_with_port: rsip::Domain::from("biloxi.example.com").into(),
//!         ..Default::default()
//!     },
//!     params: vec![rsip::Param::Tag(rsip::param::Tag::new("a73kszlfl"))],
//! };
//! ```
//!
//! ## Uri: a flexible uri struct
//!
//! As you can see in the typed `From` example, a very crucial part is the [Uri](Uri). The uri we want to generate is
//! `sips:bob@biloxi.example.com` as in the untyped header above. Hence:
//! * we specify hat the [scheme](Scheme) of the Uri is `Scheme::Sips`
//! * we specify the auth part of the Uri, but only the user part which is `Bob`. We do that
//! using the helpful [Auth](Auth) struct,
//! * we specify the host part of the Uri, which is defined in the Uri as
//! [HostWithPort](HostWithPort) struct.
//! In our case, it is a simple domain without a port specified.
//! For that we can use the [Domain](Domain) struct that has an automatic convertion to [HostWithPort](HostWithPort)
//! struct. In general you will find a lot of helpful `(Try)From/(Try)Into` convertions to remove
//! boilerplate. According to SIP spec, the URI can hold params (like `method`) and headers,
//! although in practice none really puts those to a SIP URI. That's why we have defaults in there.
//!
//! It is important to note that in order to set the `tag` param, we use the [Tag](param::Tag) struct, found
//! inside the [param](param) module. In the same module you will find various other params that can be
//! used in headers like [From](headers::From), [To](headers::To), [Contact](headers::Contact),
//! [Via](headers::Via) etc.
//!
//! In general, there are tons of helpful `(Try)From/(Try)Into` convertions to remove boilerplate.
//! So when you have a typed header, it's easy to take its untyped form:
//!
//! ```
//! # let typed_from: rsip::typed::From = rsip::typed::From {
//! #     display_name: Some("Bob".into()),
//! #     uri: rsip::Uri {
//! #         scheme: Some(rsip::Scheme::Sips),
//! #         auth: Some(rsip::Auth {
//! #             user: "Bob".into(),
//! #             password: None,
//! #         }),
//! #         host_with_port: rsip::Domain::from("biloxi.example.com").into(),
//! #         ..Default::default()
//! #     },
//! #     params: vec![rsip::Param::Tag(rsip::param::Tag::new("a73kszlfl"))],
//! # };
//! let untyped_from: rsip::headers::From = typed_from.into();
//! ```
//!
//! ## Header: the matching enum
//! [Header](Header) is the enum that holds all variants of the headers. It is there so it's easy to do
//! matches to find relevant headers.
//!
//! Continuing from the previous section, if you want to get the typed or untyped headers as part of
//! the `Header` enum you can do the following:
//! ```
//! # let typed_from: rsip::typed::From = rsip::typed::From {
//! #     display_name: Some("Bob".into()),
//! #     uri: rsip::Uri {
//! #         scheme: Some(rsip::Scheme::Sips),
//! #         auth: Some(rsip::Auth {
//! #             user: "Bob".into(),
//! #             password: None,
//! #         }),
//! #         host_with_port: rsip::Domain::from("biloxi.example.com").into(),
//! #         ..Default::default()
//! #     },
//! #     params: vec![rsip::Param::Tag(rsip::param::Tag::new("a73kszlfl"))],
//! # };
//! let from: rsip::Header = typed_from.into();
//! //or
//! # use rsip::prelude::*;
//! # let untyped_from = rsip::headers::From::new("Bob <sips:bob@biloxi.example.com>;tag=a73kszlfl");
//! let from: rsip::Header = untyped_from.into();
//! ```
//!
//! In case a header is not defined in Rsip, parsing will store it in `Header` enum in the `Other`
//! variant. For instance, constructing the `X-Fs-Sending-Message` header (related to SMS in SIP),
//! you can do:
//! ```
//! let x_fs_sending_message = rsip::Header::Other("X-FS-Sending-Message".into(), "f9c4adc8-9c2a-47d5-a7f1-63d20784685e".into());
//! ```
//! ## Headers: a vec of headers
//! [Headers](Headers) is a newtype around `Vec<Header>`, but it's there to give you better safety along
//! with some helpful methods. In SIP, many headers are allowed to appear more than once time, so
//! if we want to provide something else, maybe we should go with something similar to what the
//! http crate does (a multimap HashMap to take advantage of the characteristics of HTTP headers).
//! But this shouldn't worry you as the external API will remain the same even if the internal
//! implementation is moved from a `Vec<Header>` to a multimap HashMap.
//!
//! In order to push some headers in the [Headers](Headers) you can simple do:
//! ```
//! # use rsip::prelude::*;
//! # let from = rsip::headers::From::new("Bob <sips:bob@biloxi.example.com>;tag=a73kszlfl");
//! # let to = rsip::headers::To::new("Bob <sips:bob@biloxi.example.com>;tag=a73kszlfl");
//! let mut request_headers: rsip::Headers = Default::default();
//! request_headers.push(from.into());
//! request_headers.push(to.into());
//! //.
//! //.
//! //.
//! ```
//!
//! ## The SIP Messages: Request & Response
//! [Request](Request) & [Response](Response) are the main structs that you will use just before you finish creating
//! a SIP message. Creating those are very intuitive as you will only need to "fill-in" their
//! fields. Creating a SIP request while having already a `Headers` vec:
//! ```
//! # let request_headers = Default::default();
//! let request = rsip::Request {
//!     method: rsip::Method::Register,
//!     uri: rsip::Uri {
//!         scheme: Some(rsip::Scheme::Sips),
//!         host_with_port: rsip::Domain::from("ss2.biloxi.example.com").into(),
//!         ..Default::default()
//!     },
//!     headers: request_headers,
//!     version: rsip::Version::V2,
//!     body: vec![],
//! };
//! ```
//!
//! Now you have an [rsip::Request](Request), but sometimes it is helpful to work with the
//! [rsip::SipMessage](SipMessage) enum, especially when you are sitting on the receiver's side.
//! In order to convert a [Request](Request) (or a [Response](Response)) to a
//! [SipMessage](SipMessage) it's simple:
//!
//! ```
//! # let request_headers = Default::default();
//! # let request = rsip::Request {
//! #     method: rsip::Method::Register,
//! #     uri: rsip::Uri {
//! #         scheme: Some(rsip::Scheme::Sips),
//! #         host_with_port: rsip::Domain::from("ss2.biloxi.example.com").into(),
//! #         ..Default::default()
//! #     },
//! #     headers: request_headers,
//! #     version: rsip::Version::V2,
//! #     body: vec![],
//! # };
//! let sip_message: rsip::SipMessage = request.into();
//! //similar
//! # let request_headers = Default::default();
//! # let request = rsip::Request {
//! #     method: rsip::Method::Register,
//! #     uri: rsip::Uri {
//! #         scheme: Some(rsip::Scheme::Sips),
//! #         host_with_port: rsip::Domain::from("ss2.biloxi.example.com").into(),
//! #         ..Default::default()
//! #     },
//! #     headers: request_headers,
//! #     version: rsip::Version::V2,
//! #     body: vec![],
//! # };
//! let sip_message = rsip::SipMessage::Request(request);
//! ```
//!
//! Similarly, creating a response when having already defined the headers in a `response_headers`
//! variable:
//!
//! ```
//! # let response_headers = Default::default();
//! let response = rsip::Response {
//!     status_code: 401.into(),
//!     headers: response_headers,
//!     version: rsip::Version::V2,
//!     body: vec![],
//! };
//!
//! let sip_message: rsip::SipMessage = response.into();
//! ```
//!
//!

pub mod common;
mod error;

pub mod headers;
pub mod message;
pub mod services;
// #[cfg(feature = "tokio-codec")]
pub mod codec;

pub use error::{Error, TokenizerError};

pub use headers::{Header, Headers};
pub use message::{Request, Response, SipMessage};

pub use crate::common::uri::*;
pub use crate::common::*;

pub use crate::message::header_macros::*;

pub mod typed {
    pub use crate::headers::typed::*;
}

pub mod prelude {
    pub use crate::{
        headers::{typed::TypedHeader, ToTypedHeader, UntypedHeader},
        message::{HasHeaders, HeadersExt},
    };
}

pub(crate) type NomError<'a> = nom::Err<nom::error::VerboseError<&'a [u8]>>;
pub(crate) type NomStrError<'a> = nom::Err<nom::error::VerboseError<&'a str>>;
pub(crate) type GenericNomError<'a, T> = nom::Err<nom::error::VerboseError<T>>;
pub(crate) type IResult<'a, T> = Result<(&'a [u8], T), nom::Err<TokenizerError>>;
//pub(crate) type SResult<'a, T> = Result<(&'a str, T), nom::Err<TokenizerError>>;
pub(crate) type GResult<I, T> = Result<(I, T), nom::Err<TokenizerError>>;

//need to include &str or &[u8] in definition
pub trait AbstractInput<'a, I>:
    nom::InputTakeAtPosition<Item = I>
    + nom::InputTake
    + Clone
    + Copy
    + nom::FindSubstring<&'a str>
    + nom::Slice<nom::lib::std::ops::RangeFrom<usize>>
    + nom::InputLength
    + nom::InputIter
    + nom::Compare<&'a str>
    + nom::Offset
    + nom::Slice<core::ops::RangeTo<usize>>
    + std::fmt::Debug
    + Into<&'a bstr::BStr>
    + Default
{
    fn is_empty(&self) -> bool;
}

pub trait AbstractInputItem<I>: nom::AsChar + std::cmp::PartialEq<I> + From<u8> + Clone {
    fn is_alphabetic(c: I) -> bool;
    fn is_alphanumeric(c: I) -> bool;
    fn is_token(c: I) -> bool;
}

impl<'a> AbstractInput<'a, char> for &'a str {
    fn is_empty(&self) -> bool {
        <str>::is_empty(self)
    }
}

impl AbstractInputItem<char> for char {
    fn is_alphabetic(c: char) -> bool {
        c.is_ascii_alphabetic()
    }

    fn is_alphanumeric(c: char) -> bool {
        c.is_ascii_alphanumeric()
    }

    fn is_token(c: char) -> bool {
        Self::is_alphanumeric(c) || "-.!%*_+`'~".contains(c)
    }
}

impl<'a> AbstractInput<'a, u8> for &'a [u8] {
    fn is_empty(&self) -> bool {
        <[u8]>::is_empty(self)
    }
}
impl AbstractInputItem<u8> for u8 {
    fn is_alphabetic(c: u8) -> bool {
        nom::character::is_alphabetic(c)
    }

    fn is_alphanumeric(c: u8) -> bool {
        nom::character::is_alphanumeric(c)
    }

    fn is_token(c: u8) -> bool {
        use nom::character::is_alphanumeric;

        is_alphanumeric(c) || "-.!%*_+`'~".contains(char::from(c))
    }
}

pub(crate) mod utils {
    pub fn opt_trim(input: &str) -> Option<&str> {
        let input = input.trim();

        match input.is_empty() {
            true => None,
            false => Some(input),
        }
    }
}

pub(crate) mod parser_utils {
    use crate::TokenizerError;

    pub fn is_token(c: u8) -> bool {
        use nom::character::is_alphanumeric;

        is_alphanumeric(c) || "-.!%*_+`'~".contains(char::from(c))
    }

    pub fn is_empty_or_fail_with<'a, I, T: crate::AbstractInput<'a, I>, S: Into<&'a bstr::BStr>>(
        rem: T,
        tuple: (&'static str, S),
    ) -> Result<(), nom::Err<crate::TokenizerError>> {
        if !rem.is_empty() {
            //TODO: specify that this is trailing input
            //use a comma in params tests to test
            Err(TokenizerError::from(tuple).into())
        } else {
            Ok(())
        }
    }

    /*
    pub fn create_error_for<'a>(rem: &'a [u8], error: &'static str) -> super::NomError<'a> {
        nom::Err::Error(nom::error::VerboseError {
            errors: vec![(rem, nom::error::VerboseErrorKind::Context(error))],
        })
    }
    */

    /*
        pub fn is_unreserved(chr: u8) -> bool {
            use nom::character::is_alphanumeric;

            is_alphanumeric(chr) || "-_.!~*'()".contains(char::from(chr))
        }

        pub fn is_reserved(chr: u8) -> bool {
            ";/?:@&=+$,".contains(char::from(chr))
        }
    */

    /*
        pub fn opt_sc<'a>(
            input: &'a [u8],
        ) -> IResult<&'a [u8], Option<&'a [u8]>, VerboseError<&'a [u8]>> {
            use nom::{bytes::complete::tag, combinator::opt};

            opt(tag(";"))(input)
        }

        pub fn opt_amp<'a>(
            input: &'a [u8],
        ) -> IResult<&'a [u8], Option<&'a [u8]>, VerboseError<&'a [u8]>> {
            use nom::{bytes::complete::tag, combinator::opt};

            opt(tag("&"))(input)
        }
    */
}
