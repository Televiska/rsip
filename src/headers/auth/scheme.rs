pub use tokenizer::Tokenizer;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Scheme {
    Digest,
    Other(String),
}

impl Default for Scheme {
    fn default() -> Self {
        Self::Digest
    }
}

impl std::fmt::Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Digest => write!(f, "Digest"),
            Self::Other(inner) => write!(f, "{}", inner),
        }
    }
}

impl<'a> std::convert::TryFrom<Tokenizer<'a, &'a [u8]>> for Scheme {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer<'a, &'a [u8]>) -> Result<Self, Self::Error> {
        use std::str::from_utf8;

        match from_utf8(tokenizer.value)? {
            part if part.eq_ignore_ascii_case("digest") => Ok(Self::Digest),
            part => Ok(Self::Other(part.into())),
        }
    }
}

impl<'a> std::convert::TryFrom<Tokenizer<'a, &'a str>> for Scheme {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer<'a, &'a str>) -> Result<Self, Self::Error> {
        match tokenizer.value {
            part if part.eq_ignore_ascii_case("digest") => Ok(Self::Digest),
            part => Ok(Self::Other(part.into())),
        }
    }
}

mod tokenizer {
    use crate::AbstractInput;
    use crate::GenericNomError;
    use std::marker::PhantomData;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Tokenizer<'a, T>
    where
        T: AbstractInput<'a>,
    {
        pub value: T,
        phantom: PhantomData<&'a T>,
    }

    impl<'a, T> From<T> for Tokenizer<'a, T>
    where
        T: AbstractInput<'a>,
    {
        fn from(value: T) -> Self {
            Self {
                value,
                phantom: PhantomData,
            }
        }
    }

    impl<'a, T> Tokenizer<'a, T>
    where
        T: AbstractInput<'a>,
    {
        pub fn tokenize(part: T) -> Result<(T, Self), GenericNomError<'a, T>> {
            use nom::{branch::alt, bytes::complete::take_until, combinator::rest};

            let (rem, scheme) = alt((take_until(" "), rest))(part)?;

            Ok((rem, scheme.into()))
        }
    }
}
