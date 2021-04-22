pub mod common;
mod error;
pub mod headers;
pub mod message;

pub use error::Error;
pub use headers::{Header, Headers};
pub use message::{Request, Response, SipMessage};

pub(crate) mod parser_utils {
    use nom::{error::VerboseError, IResult};

    pub(crate) fn opt_sp<'a>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Option<&'a [u8]>, VerboseError<&'a [u8]>> {
        use nom::{bytes::complete::tag, combinator::opt};

        opt(tag(" "))(input)
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
