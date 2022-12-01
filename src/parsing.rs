use std::num::ParseIntError;
use std::fmt::{self, Display, Formatter};
use std::io;

pub use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    ParseInt(ParseIntError),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            ParseError::Io(e) => write!(f, "Failed to parse input: {}", e),
            ParseError::ParseInt(e) => write!(f, "Failed to parse input: {}", e),
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(e: io::Error) -> Self {
        ParseError::Io(e)
    }
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::ParseInt(e)
    }
}
