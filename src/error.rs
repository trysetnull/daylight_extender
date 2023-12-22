use std::{error::Error, fmt};

#[derive(Debug)]
pub enum CustomError<'a> {
    ChronoError(&'a str),
}

impl<'a> Error for CustomError<'a> {}

impl<'a> fmt::Display for CustomError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::ChronoError(msg) => write!(f, "{msg}"),
        }
    }
}
