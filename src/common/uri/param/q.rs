use rsip_derives::{IntoParam, NewType};

/// Simple NewType around String. Intended to be used for the `q` parameter found in the `Contact`
/// header.
//TODO: add typed + default
#[derive(NewType, IntoParam, Debug, PartialEq, Eq, Clone)]
pub struct Q(String);

#[cfg(feature = "test-utils")]
impl testing_utils::Randomize for Q {
    fn random() -> Self {
        Self(format!("0.{}", testing_utils::rand_num_from(1..=9)))
    }
}
