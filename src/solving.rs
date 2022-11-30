use std::fmt::{self, Display, Formatter};
use std::io::{BufReader, Read};

use crate::parsing::ParseError;

pub struct SolveError {
    day: u8,
    part: u8,
    reason: SolveErrorReason,
}

impl Display for SolveError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "Failed to solve day {} part {}: {}", self.day, self.part, self.reason)
    }
}

pub enum SolveErrorReason {
    Unimplemented,
}

impl Display for SolveErrorReason {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            SolveErrorReason::Unimplemented => write!(f, "Not implemented"),
        }
    }
}

pub type SolveResult = Result<u64, SolveError>;

pub trait Solver {
    type Input;
    const DAY: u8;

    fn parse<R: Read>(input: BufReader<R>) -> Result<Self::Input, ParseError>;

    fn part_1(_input: &Self::Input) -> SolveResult {
        Err(SolveError { day: Self::DAY, part: 1, reason: SolveErrorReason::Unimplemented })
    }

    fn part_2(_input: &Self::Input) -> SolveResult {
        Err(SolveError { day: Self::DAY, part: 2, reason: SolveErrorReason::Unimplemented })
    }
}
