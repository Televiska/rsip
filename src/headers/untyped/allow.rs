use rsip_derives::UntypedHeader;

#[derive(UntypedHeader, Debug, PartialEq, Eq, Clone)]
pub struct Allow(String);

impl Default for Allow {
    fn default() -> Self {
        Self(
            crate::common::Method::all()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )
    }
}
