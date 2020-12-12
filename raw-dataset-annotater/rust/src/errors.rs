use serde_json::Error as SerdeError;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    General(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::General(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::General(err.to_string())
    }
}

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Error {
        Error::General(err.to_string())
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Error {
        Error::General(msg.to_string())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::General(msg) => msg,
        }
    }
}
