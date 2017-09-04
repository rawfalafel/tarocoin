use std::error::Error as StdError;
use std::io;
use std::fmt;

extern crate secp256k1;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    Secp256k1Failure(secp256k1::Error),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IOError(_) => "IOError",
            Error::Secp256k1Failure(_) => "Secp256k1Failure",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::IOError(ref err) => Some(err as &StdError),
            Error::Secp256k1Failure(ref err) => Some(err as &StdError)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IOError(error)
    }
}

impl From<secp256k1::Error> for Error {
    fn from(error: secp256k1::Error) -> Self {
        Error::Secp256k1Failure(error)
    }
}

