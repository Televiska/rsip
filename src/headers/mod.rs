pub mod auth;
pub mod header;
pub mod typed;
pub mod untyped;
pub use header::Header;
pub use untyped::*;

/// Simple NewType around `Vec<Header>` that gives many helpful methods when dealing with headers
/// in [super::Request], [super::Response] and [super::SipMessage].
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Headers(Vec<Header>);

impl Headers {
    pub fn push(&mut self, h: Header) {
        self.0.push(h)
    }

    pub fn unique_push(&mut self, h: Header) {
        self.0
            .retain(|s| std::mem::discriminant(s) != std::mem::discriminant(&h));
        self.push(h);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Header> {
        self.0.iter()
    }

    pub fn extend(&mut self, i: Vec<Header>) {
        self.0.extend(i)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Header> {
        self.0.iter_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Header) -> bool,
    {
        self.0.retain(f)
    }
}

impl IntoIterator for Headers {
    type IntoIter = ::std::vec::IntoIter<Self::Item>;
    type Item = Header;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::convert::From<Header> for Headers {
    fn from(header: Header) -> Self {
        Self(vec![header])
    }
}

impl std::convert::From<Vec<Header>> for Headers {
    fn from(headers: Vec<Header>) -> Self {
        Self(headers)
    }
}

impl std::convert::From<Headers> for Vec<Header> {
    fn from(from: Headers) -> Self {
        from.0
    }
}

impl std::fmt::Display for Headers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "")
        } else {
            write!(
                f,
                "{}\r\n",
                self.iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join("\r\n")
            )
        }
    }
}
