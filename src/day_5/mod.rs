mod parsing;

use crate::{ParseError, SolveError};

pub type Crates = Vec<Vec<char>>;

#[derive(Debug, PartialEq)]
pub struct Task {
    pub num: usize,
    pub from: usize,
    pub to: usize,
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = (Crates, Vec<Task>);
    type Output = String;
    const DAY: u8 = 5;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        parsing::parse(input)
    }

    fn part_1(input: Self::Input) -> Result<Self::Output, SolveError> {
        let (mut crates, tasks) = input;

        for task in tasks {
            for _ in 0..task.num {
                let top = crates[task.from - 1]
                    .pop()
                    .ok_or(SolveError::InvalidInput)?;
                crates[task.to - 1].push(top);
            }
        }

        Ok(to_string(&crates))
    }

    fn part_2(input: Self::Input) -> Result<Self::Output, SolveError> {
        let (mut crates, tasks) = input;

        for task in tasks {
            let len = crates[task.from - 1].len();
            let moving_crates = crates[task.from - 1].split_off(len - task.num);
            crates[task.to - 1].extend(moving_crates.into_iter());
        }

        Ok(to_string(&crates))
    }
}

fn to_string(crates: &Crates) -> String {
    crates
        .iter()
        .map(|stack| stack.last().unwrap_or(&' '))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Solver;

    pub fn get_input() -> (Crates, Vec<Task>) {
        let crates = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

        let tasks = vec![
            Task {
                num: 1,
                from: 2,
                to: 1,
            },
            Task {
                num: 3,
                from: 1,
                to: 3,
            },
            Task {
                num: 2,
                from: 2,
                to: 1,
            },
            Task {
                num: 1,
                from: 1,
                to: 2,
            },
        ];

        (crates, tasks)
    }

    #[test]
    fn part_1() {
        let result = super::Solver::part_1(get_input()).unwrap();

        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part_2() {
        let result = super::Solver::part_2(get_input()).unwrap();

        assert_eq!(result, "MCD");
    }
}
