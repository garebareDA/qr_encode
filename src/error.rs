use std::error;
use std::fmt;


#[derive(Debug)]
pub enum  Error {
    InvalidEncode(String)
}

impl  fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidEncode(ref s) => write!(f, "Invalid encode: {}", s),
        }
    }
}

impl error::Error for Error {}