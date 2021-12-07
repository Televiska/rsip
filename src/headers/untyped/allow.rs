use rsip_derives::{ToTypedHeader, UntypedHeader};

/// The `Allow` header in its [untyped](super) form.
#[derive(UntypedHeader, ToTypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Allow(String);

impl Default for Allow {
    fn default() -> Self {
        Self(
            crate::common::Method::all()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}
