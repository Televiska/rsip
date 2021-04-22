pub mod branch;
pub mod maddr;
pub mod method;
pub mod ttl;
pub mod user;

pub use branch::Branch;
pub use maddr::Maddr;
pub use method::Method;
pub use ttl::Ttl;
pub use user::User;

use crate::common::{uri::HostWithPort, Transport};
use macros::{Display, FromIntoInner, FromStr, HasValue};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Param {
    Branch(Branch),
    Lr(Method),
    Maddr(Maddr),
    Method(Method),
    RPort(Option<u16>),
    Received(HostWithPort),
    Transport(Transport),
    Ttl(Ttl),
    User(User),
    Other(OtherParam, OtherParamValue),
}

#[derive(HasValue, Display, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct OtherParam(String);
#[derive(HasValue, Display, FromIntoInner, FromStr, Debug, PartialEq, Eq, Clone)]
pub struct OtherParamValue(String);
