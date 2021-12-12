use rsip_derives::{IntoParam, NewType};

/// Simple NewType around String. Intended to be used for the `branch` parameter found in the `Via`
/// header.
///
/// Provides a simple default implementation that uses a `Uuid` for genearting a unique branch
/// across space & time.
#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct Branch(String);

impl<'a> Default for Branch {
    fn default() -> Self {
        use uuid::Uuid;
        Self::new(format!("z9hG4bK-rsip-{}", Uuid::new_v4()))
    }
}

#[cfg(feature = "test-utils")]
impl testing_utils::Randomize for Branch {
    fn random() -> Self {
        Self::default()
    }
}
