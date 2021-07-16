pub mod common;
mod error;
pub mod headers;
pub mod message;
pub mod services;

pub use error::Error;
pub use headers::{Header, Headers};
pub use message::{Request, Response, SipMessage};

pub mod prelude {
    pub mod rsip {
        pub use crate::*;
        pub use message::header_macros::*;

        pub mod headers {
            pub use crate::headers::*;
        }

        pub mod typed {
            pub use crate::headers::typed::*;
        }
    }

    pub use crate::{
        headers::{typed::TypedHeader, ToTypedHeader, UntypedHeader},
        message::{HasHeaders, HeadersExt},
    };
}

pub(crate) type NomError<'a> = nom::Err<nom::error::VerboseError<&'a [u8]>>;
pub(crate) type GenericNomError<'a, T> = nom::Err<nom::error::VerboseError<T>>;

pub trait AbstractInput<'a>:
    nom::InputTakeAtPosition
    + nom::InputTake
    + Clone
    + nom::FindSubstring<&'a str>
    + nom::Slice<nom::lib::std::ops::RangeFrom<usize>>
    + nom::InputLength
    + nom::InputIter
    + nom::Compare<&'a str>
{
}

impl<'a, T> AbstractInput<'a> for T where
    T: nom::InputTakeAtPosition
        + nom::InputTake
        + Clone
        + nom::FindSubstring<&'a str>
        + nom::Slice<nom::lib::std::ops::RangeFrom<usize>>
        + nom::InputLength
        + nom::InputIter
        + nom::Compare<&'a str>
{
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
    use nom::{error::VerboseError, IResult};

    pub fn opt_sp(input: &[u8]) -> IResult<&[u8], Option<&[u8]>, VerboseError<&[u8]>> {
        use nom::{bytes::complete::tag, combinator::opt};

        opt(tag(" "))(input)
    }

    pub fn create_error_for<'a>(rem: &'a [u8], error: &'static str) -> super::NomError<'a> {
        nom::Err::Error(nom::error::VerboseError {
            errors: vec![(rem, nom::error::VerboseErrorKind::Context(error))],
        })
    }

    pub fn is_token(chr: u8) -> bool {
        use nom::character::is_alphanumeric;

        is_alphanumeric(chr) || "-.!%*_+`'~".contains(char::from(chr))
    }

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
