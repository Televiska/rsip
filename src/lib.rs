pub mod common;
mod error;
pub mod headers;
pub mod message;

pub use error::Error;
pub use headers::{Header, Headers};
pub use message::{Request, Response, SipMessage};

pub(crate) type NomError<'a> = nom::Err<nom::error::VerboseError<&'a [u8]>>;

pub(crate) mod parser_utils {
    use nom::{error::VerboseError, IResult};

    pub(crate) fn opt_sp(input: &[u8]) -> IResult<&[u8], Option<&[u8]>, VerboseError<&[u8]>> {
        use nom::{bytes::complete::tag, combinator::opt};

        opt(tag(" "))(input)
    }

    pub(crate) fn create_error_for<'a>(rem: &'a [u8], error: &'static str) -> super::NomError<'a> {
        nom::Err::Error(nom::error::VerboseError {
            errors: vec![(rem, nom::error::VerboseErrorKind::Context(error))],
        })
    }
    /*
        pub(crate) fn opt_sc<'a>(
            input: &'a [u8],
        ) -> IResult<&'a [u8], Option<&'a [u8]>, VerboseError<&'a [u8]>> {
            use nom::{bytes::complete::tag, combinator::opt};

            opt(tag(";"))(input)
        }

        pub(crate) fn opt_amp<'a>(
            input: &'a [u8],
        ) -> IResult<&'a [u8], Option<&'a [u8]>, VerboseError<&'a [u8]>> {
            use nom::{bytes::complete::tag, combinator::opt};

            opt(tag("&"))(input)
        }
    */
}
