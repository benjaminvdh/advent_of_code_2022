mod parsing;
mod rock_paper_scissors;

use crate::{ParseError, SolveResult};

type Input = Vec<rock_paper_scissors::Round>;

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Input;
    const DAY: u8 = 2;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        parsing::parse(input)
    }

    fn part_1(input: Self::Input) -> SolveResult {
        Ok(input.iter().fold(0, |acc, round| acc + round.get_score()))
    }

    fn part_2(input: Self::Input) -> SolveResult {
        Ok(input
            .iter()
            .map(|round| round.to_strategy())
            .fold(0, |acc, round| acc + round.get_score()))
    }
}
