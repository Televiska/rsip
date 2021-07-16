use rsip_derives::NewType;

#[derive(NewType, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Port(u16);

impl Default for Port {
    fn default() -> Self {
        Self(5060)
    }
}

impl std::convert::TryFrom<String> for Port {
    type Error = crate::Error;

    fn try_from(from: String) -> Result<Self, Self::Error> {
        use std::convert::TryInto;

        from.as_str().try_into()
    }
}

impl std::convert::TryFrom<&str> for Port {
    type Error = crate::Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Ok(from.parse::<u16>()?.into())
    }
}
