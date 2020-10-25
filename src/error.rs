#![allow(dead_code)]

use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    MissingHeader
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingHeader => write!(f, "Libsip error: missing header"),
        }
    }
}

impl StdError for Error {}
