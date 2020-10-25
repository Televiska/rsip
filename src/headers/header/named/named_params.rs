use super::NamedParamTrait;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NamedParams<T: NamedParamTrait>(pub Vec<T>);

impl<T: NamedParamTrait> NamedParams<T> {
    pub fn push(&mut self, h: T) {
        self.0.push(h)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    pub fn extend(&mut self, i: Vec<T>) {
        self.0.extend(i)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.0.iter_mut()
    }
}

impl<T: NamedParamTrait> IntoIterator for NamedParams<T> {
    type IntoIter = ::std::vec::IntoIter<Self::Item>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: NamedParamTrait> Default for NamedParams<T> {
    fn default() -> Self {
        Self(vec![])
    }
}

impl<T: NamedParamTrait> Into<HashMap<String, Option<String>>> for NamedParams<T> {
    fn into(self) -> HashMap<String, Option<String>> {
        self.0
            .into_iter()
            .map(Into::<(String, Option<String>)>::into)
            .collect::<_>()
    }
}

impl<T: NamedParamTrait> From<HashMap<String, Option<String>>> for NamedParams<T> {
    fn from(from: HashMap<String, Option<String>>) -> Self {
        from.into_iter()
            .map(Into::<T>::into)
            .collect::<Vec<T>>()
            .into()
    }
}

impl<T: NamedParamTrait> From<Vec<T>> for NamedParams<T> {
    fn from(from: Vec<T>) -> Self {
        Self(from)
    }
}
/*
impl<T: NamedParamTrait> From<T> for NamedParams<T> {
    fn from(from: T) -> Self {
        Self(vec![from])
    }
}*/

impl<T: NamedParamTrait + Into<(String, Option<libsip::headers::GenValue>)>>
    Into<HashMap<String, Option<libsip::headers::GenValue>>> for NamedParams<T>
{
    fn into(self) -> HashMap<String, Option<libsip::headers::GenValue>> {
        self.0
            .into_iter()
            .map(Into::<(String, Option<libsip::headers::GenValue>)>::into)
            .collect::<HashMap<String, Option<libsip::headers::GenValue>>>()
    }
}
