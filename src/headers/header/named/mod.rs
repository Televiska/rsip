mod contact_param;
mod named_header;
mod named_param;
mod named_params;

pub use contact_param::{ContactParam, GenValue};
pub use named_header::NamedHeader;
pub use named_param::{NamedParam, Tag};
pub use named_params::NamedParams;

pub trait NamedParamTrait:
    Default + Into<(String, Option<String>)> + From<(String, Option<String>)>
{
}
impl<T: Default + Into<(String, Option<String>)> + From<(String, Option<String>)>> NamedParamTrait
    for T
{
}
