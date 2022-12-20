use crate::{ParseError, SolveError};

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Vec<i32>;
    type Output = i32;
    const DAY: u8 = 20;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        Ok(input
            .lines()
            .map(|line| line.parse::<i32>())
            .collect::<Result<_, _>>()?)
    }

    fn part_1(input: Self::Input) -> Result<Self::Output, SolveError> {
        let mut input: Vec<_> = input.into_iter().enumerate().collect();
        let original_input = input.clone();

        for (index, number) in original_input {
            let position = input
                .iter()
                .position(|(i, n)| (*i, *n) == (index, number))
                .unwrap();

            let _ = input.remove(position);

            let mut new_position =
                (position as i32 + number).rem_euclid(input.len() as i32) as usize;

            if new_position == 0 {
                new_position = input.len();
            }

            input.insert(new_position, (index, number));
        }

        let index = input
            .iter()
            .position(|(_, number)| *number == 0)
            .ok_or(SolveError::InvalidInput)? as usize;

        let n1 = input[(index + 1000) % input.len()].1;
        let n2 = input[(index + 2000) % input.len()].1;
        let n3 = input[(index + 3000) % input.len()].1;

        Ok(n1 + n2 + n3)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    fn get_input() -> Vec<i32> {
        vec![1, 2, -3, 3, -2, 0, 4]
    }

    #[test]
    fn parsing() {
        let input = r"1
2
-3
3
-2
0
4";

        assert_eq!(
            super::Solver::parse(String::from(input)).unwrap(),
            get_input()
        );
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(super::Solver::part_1(input).unwrap(), 3);
    }
}
