use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{self, BufReader};

#[derive(Debug)]
pub enum InputError {
    Io(io::Error),
    NoInputSpecified
}

impl From<io::Error> for InputError {
    fn from(e: io::Error) -> Self {
        InputError::Io(e)
    }
}

impl Display for InputError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            InputError::Io(e) => e.fmt(f),
            InputError::NoInputSpecified => write!(f, "No input file specified."),
        }
    }
}

pub fn get_input() -> Result<BufReader<File>, InputError> {
    let arg = env::args_os().nth(1).ok_or(InputError::NoInputSpecified)?;
    let file = File::open(arg)?;
    Ok(BufReader::new(file))
}
