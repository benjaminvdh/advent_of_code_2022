use std::fmt::{self, Display, Formatter};

use crate::parsing::ParseError;
use crate::AocError;

pub type SolveResult = Result<u64, SolveError>;

pub trait Solver {
    type Input;
    const DAY: u8;

    fn parse(input: String) -> Result<Self::Input, ParseError>;

    fn part_1(_input: Self::Input) -> SolveResult {
        Err(SolveError::Unimplemented)
    }

    fn part_2(_input: Self::Input) -> SolveResult {
        Err(SolveError::Unimplemented)
    }
}

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

impl From<SolveError> for AocError {
    fn from(e: SolveError) -> Self {
        AocError::Solving(e)
    }
}
