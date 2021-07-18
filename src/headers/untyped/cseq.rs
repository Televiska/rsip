use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `CSeq` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
#[header(display_name = "CSeq")]
pub struct CSeq(String);
