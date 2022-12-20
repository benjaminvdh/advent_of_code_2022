use crate::{ParseError, SolveError};

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Vec<i64>;
    type Output = i64;
    const DAY: u8 = 20;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        Ok(input
            .lines()
            .map(|line| line.parse::<i64>())
            .collect::<Result<_, _>>()?)
    }

    fn part_1(input: Self::Input) -> Result<Self::Output, SolveError> {
        mix(input, 1, 1)
    }

    fn part_2(input: Self::Input) -> Result<Self::Output, SolveError> {
        mix(input, 811589153, 10)
    }
}

fn mix(input: Vec<i64>, factor: i64, repetitions: usize) -> Result<i64, SolveError> {
    let mut input: Vec<_> = input
        .into_iter()
        .map(|num| num * factor)
        .enumerate()
        .collect();
    let original_input = input.clone();

    for _ in 0..repetitions {
        for (index, number) in original_input.iter() {
            let position = input
                .iter()
                .position(|(i, n)| (i, n) == (index, number))
                .unwrap();

            let _ = input.remove(position);

            let new_position = (position as i64 + number).rem_euclid(input.len() as i64) as usize;

            input.insert(new_position, (*index, *number));
        }
    }

    let index = input
        .iter()
        .position(|(_, number)| *number == 0)
        .ok_or(SolveError::InvalidInput)? as usize;

    let (_, n1) = input[(index + 1000) % input.len()];
    let (_, n2) = input[(index + 2000) % input.len()];
    let (_, n3) = input[(index + 3000) % input.len()];

    Ok(n1 + n2 + n3)
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    fn get_input() -> Vec<i64> {
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

    #[test]
    fn part_2() {
        let input = get_input();

        assert_eq!(super::Solver::part_2(input).unwrap(), 1623178306);
    }
}
