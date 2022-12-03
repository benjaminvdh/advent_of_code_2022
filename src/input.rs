use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io;

use crate::AocError;

pub fn get_input_file() -> Result<File, AocError> {
    let arg = env::args_os().nth(1).ok_or(InputError::NoInputSpecified)?;
    let file = File::open(arg).map_err(|e| InputError::from(e))?;

    Ok(file)
}

#[derive(Debug)]
pub enum InputError {
    Io(io::Error),
    NoInputSpecified,
}

impl Display for InputError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            InputError::Io(e) => e.fmt(f),
            InputError::NoInputSpecified => write!(f, "No input file specified."),
        }
    }
}

impl From<io::Error> for InputError {
    fn from(e: io::Error) -> Self {
        InputError::Io(e)
    }
}

impl From<InputError> for AocError {
    fn from(e: InputError) -> Self {
        AocError::Input(e)
    }
}
