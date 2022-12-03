use std::fmt::{self, Display, Formatter};
pub use std::io::{BufReader, Read};

pub use crate::parsing::ParseError;

#[derive(Debug)]
pub enum SolveError {
    EmptyInput,
    InvalidInput,
    Unimplemented,
}

impl Display for SolveError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            SolveError::EmptyInput => write!(f, "Input is empty"),
            SolveError::InvalidInput => write!(f, "Input is invalid"),
            SolveError::Unimplemented => write!(f, "Not implemented"),
        }
    }
}

pub type SolveResult = Result<u64, SolveError>;

pub trait Solver {
    type Input;
    const DAY: u8;

    fn parse<R: Read>(input: BufReader<R>) -> Result<Self::Input, ParseError>;

    fn part_1(_input: Self::Input) -> SolveResult {
        Err(SolveError::Unimplemented)
    }

    fn part_2(_input: Self::Input) -> SolveResult {
        Err(SolveError::Unimplemented)
    }
}
