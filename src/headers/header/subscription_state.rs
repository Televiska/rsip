use crate::headers::Header;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SubscriptionState {
    Active(Active),
    Pending(Pending),
    Terminated(Terminated),
    Other(Other),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Active {
    expires: Option<u32>,
    parameters: HashMap<String, Option<String>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pending {
    expires: Option<u32>,
    parameters: HashMap<String, Option<String>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Terminated {
    retry_after: Option<u32>,
    reason: Option<String>,
    parameters: HashMap<String, Option<String>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Other {
    state: String,
    parameters: HashMap<String, Option<String>>,
}

impl Into<Header> for SubscriptionState {
    fn into(self) -> Header {
        Header::SubscriptionState(self)
    }
}

impl Into<libsip::headers::SubscriptionState> for Active {
    fn into(self) -> libsip::headers::SubscriptionState {
        libsip::headers::SubscriptionState::Active {
            expires: self.expires,
            parameters: self.parameters,
        }
    }
}

impl Into<libsip::headers::SubscriptionState> for Pending {
    fn into(self) -> libsip::headers::SubscriptionState {
        libsip::headers::SubscriptionState::Pending {
            expires: self.expires,
            parameters: self.parameters,
        }
    }
}

impl Into<libsip::headers::SubscriptionState> for Terminated {
    fn into(self) -> libsip::headers::SubscriptionState {
        libsip::headers::SubscriptionState::Terminated {
            retry_after: self.retry_after,
            reason: self.reason,
            parameters: self.parameters,
        }
    }
}

impl Into<libsip::headers::SubscriptionState> for Other {
    fn into(self) -> libsip::headers::SubscriptionState {
        libsip::headers::SubscriptionState::Other {
            state: self.state,
            parameters: self.parameters,
        }
    }
}

impl Into<libsip::headers::SubscriptionState> for SubscriptionState {
    fn into(self) -> libsip::headers::SubscriptionState {
        match self {
            Self::Active(inner) => inner.into(),
            Self::Pending(inner) => inner.into(),
            Self::Terminated(inner) => inner.into(),
            Self::Other(inner) => inner.into(),
        }
    }
}

impl From<libsip::headers::SubscriptionState> for SubscriptionState {
    fn from(from: libsip::headers::SubscriptionState) -> Self {
        match from {
            libsip::headers::SubscriptionState::Active {
                expires,
                parameters,
            } => Self::Active(Active {
                expires,
                parameters,
            }),
            libsip::headers::SubscriptionState::Pending {
                expires,
                parameters,
            } => Self::Pending(Pending {
                expires,
                parameters,
            }),
            libsip::headers::SubscriptionState::Terminated {
                retry_after,
                reason,
                parameters,
            } => Self::Terminated(Terminated {
                retry_after,
                reason,
                parameters,
            }),
            libsip::headers::SubscriptionState::Other { state, parameters } => {
                Self::Other(Other { state, parameters })
            }
        }
    }
}

impl Into<libsip::headers::Header> for SubscriptionState {
    fn into(self) -> libsip::headers::Header {
        libsip::headers::Header::SubscriptionState(self.into())
    }
}
