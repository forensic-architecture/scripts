use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub struct GeneralError {
    pub msg: String,
}

impl GeneralError {
    pub fn new(msg: &str) -> GeneralError {
        GeneralError {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<io::Error> for GeneralError {
    fn from(err: io::Error) -> GeneralError {
        GeneralError::new(&err.to_string())
    }
}

impl Error for GeneralError {
    fn description(&self) -> &str {
        &self.msg
    }
}
