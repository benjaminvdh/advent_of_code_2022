use std::io::{BufReader, Read};

use crate::parsing::ParseError;
use crate::solving::{SolveError, SolveResult};

type Input = Vec<u64>;

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Input;
    const DAY: u8 = 1;

    fn parse<R: Read>(mut input: BufReader<R>) -> Result<Self::Input, ParseError> {
        let mut contents = String::new();
        let _ = input.read_to_string(&mut contents)?;

        let elves = contents
            .split("\n\n")
            .map(|elf| elf.lines().map(|line| line.parse::<u64>()).sum())
            .collect::<Result<_, _>>()?;

        Ok(elves)
    }

    fn part_1(input: Self::Input) -> SolveResult {
        input.into_iter().max().ok_or(SolveError::EmptyInput)
    }

    fn part_2(mut input: Self::Input) -> SolveResult {
        input.sort_unstable();
        Ok(input.iter().rev().take(3).sum())
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::solving::Solver;

    fn get_input() -> Vec<u64> {
        vec![6000, 4000, 11000, 24000, 10000]
    }

    #[test]
    fn parsing() {
        let input = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let parsed_input = super::Solver::parse(BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(parsed_input, get_input());
    }

    #[test]
    fn part_1() {
        assert_eq!(super::Solver::part_1(get_input()).unwrap(), 24000);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::Solver::part_2(get_input()).unwrap(), 45000);
    }
}
