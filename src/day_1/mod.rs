use crate::{ParseError, SolveError, SolveResult};

type Input = Vec<u64>;

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Input;
    const DAY: u8 = 1;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        let elves = input
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

        let parsed_input = super::Solver::parse(String::from(input)).unwrap();
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
