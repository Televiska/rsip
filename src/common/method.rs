#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Method {
    Invite,
    Ack,
    Bye,
    Cancel,
    Register,
    Options,
    PRack,
    Subscribe,
    Notify,
    Publish,
    Info,
    Refer,
    Message,
    Update,
}

impl Default for Method {
    fn default() -> Method {
        Method::Register
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<libsip::core::Method>::into(self.clone()))
    }
}

impl Into<libsip::core::Method> for Method {
    fn into(self) -> libsip::core::Method {
        match self {
            Self::Invite => libsip::core::Method::Invite,
            Self::Ack => libsip::core::Method::Ack,
            Self::Bye => libsip::core::Method::Bye,
            Self::Cancel => libsip::core::Method::Cancel,
            Self::Register => libsip::core::Method::Register,
            Self::Options => libsip::core::Method::Options,
            Self::PRack => libsip::core::Method::PRack,
            Self::Subscribe => libsip::core::Method::Subscribe,
            Self::Notify => libsip::core::Method::Notify,
            Self::Publish => libsip::core::Method::Publish,
            Self::Info => libsip::core::Method::Info,
            Self::Refer => libsip::core::Method::Refer,
            Self::Message => libsip::core::Method::Message,
            Self::Update => libsip::core::Method::Update,
        }
    }
}

impl From<libsip::core::Method> for Method {
    fn from(from: libsip::core::Method) -> Method {
        match from {
            libsip::core::Method::Invite => Self::Invite,
            libsip::core::Method::Ack => Self::Ack,
            libsip::core::Method::Bye => Self::Bye,
            libsip::core::Method::Cancel => Self::Cancel,
            libsip::core::Method::Register => Self::Register,
            libsip::core::Method::Options => Self::Options,
            libsip::core::Method::PRack => Self::PRack,
            libsip::core::Method::Subscribe => Self::Subscribe,
            libsip::core::Method::Notify => Self::Notify,
            libsip::core::Method::Publish => Self::Publish,
            libsip::core::Method::Info => Self::Info,
            libsip::core::Method::Refer => Self::Refer,
            libsip::core::Method::Message => Self::Message,
            libsip::core::Method::Update => Self::Update,
        }
    }
}
