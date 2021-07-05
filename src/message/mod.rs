#[macro_use]
pub mod header_macros;
pub mod headers_ext;
pub mod request;
pub mod response;
pub mod sip_message;

pub use headers_ext::HeadersExt;
pub use request::Request;
pub use response::Response;
pub use sip_message::{SipMessage, Tokenizer};

pub trait HasHeaders {
    fn headers(&self) -> &crate::headers::Headers;
    fn headers_mut(&mut self) -> &mut crate::headers::Headers;
}
