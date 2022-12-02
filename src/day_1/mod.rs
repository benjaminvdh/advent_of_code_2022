mod parsing;
mod part_1;
mod part_2;

use crate::solving::*;

type Input = Vec<Vec<u64>>;

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Input;
    const DAY: u8 = 1;

    fn parse<R: Read>(input: BufReader<R>) -> Result<Self::Input, ParseError> {
        parsing::parse(input)
    }

    fn part_1(input: Self::Input) -> SolveResult {
        part_1::solve(input)
    }

    fn part_2(input: Self::Input) -> SolveResult {
        part_2::solve(input)
    }
}
