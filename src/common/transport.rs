#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Transport {
    Udp,
    Tcp,
}

impl Default for Transport {
    fn default() -> Self {
        Self::Udp
    }
}
