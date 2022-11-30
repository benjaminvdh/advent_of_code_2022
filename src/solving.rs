use std::fmt::{self, Display, Formatter};
use std::io::{BufReader, Read};

use crate::parsing::ParseError;

pub struct SolveError {
    day: u8,
    part: u8,
    descr: String,
}

impl Display for SolveError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "Failed to solve day {} part {}: {}", self.day, self.part, self.descr)
    }
}

pub type SolveResult = Result<u64, SolveError>;

pub trait Solver {
    type Input;
    const DAY: u8;

    fn parse<R: Read>(input: BufReader<R>) -> Result<Self::Input, ParseError>;

    fn part_1(input: &Self::Input) -> SolveResult;
    fn part_2(input: &Self::Input) -> SolveResult;
}
