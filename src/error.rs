use std::error;
use std::fmt;
use std::io;


#[derive(Debug)]
pub enum  Error {
    InvalidEncode(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::InvalidEncode(err.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidEncode(ref s) => write!(f, "Invalid encode: {}", s),
        }
    }
}

impl error::Error for Error {}