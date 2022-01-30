pub mod typed;

use rsip::headers::InReplyTo;

validate_untyped_header_trait!(InReplyTo);
validate_to_typed_header_trait!(InReplyTo);
