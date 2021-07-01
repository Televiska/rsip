#[macro_use]
pub mod header_macros;
//mod debug_ext;
//mod expires_ext;
pub mod message_ext;
pub mod request;
pub mod response;
pub mod sip_message;

//pub use debug_ext::DebugExt;
//pub use expires_ext::ExpiresExt;
pub use message_ext::MessageExt;
pub use request::Request;
pub use response::Response;
pub use sip_message::{SipMessage, Tokenizer};

pub trait HasHeaders {
    fn headers(&self) -> &crate::headers::Headers;
    fn headers_mut(&mut self) -> &mut crate::headers::Headers;
}
