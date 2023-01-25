use std::iter::FromIterator;

use crate::{ParseError, SolveError};

fn parse(c: char) -> Result<i64, SolveError> {
    match c {
        '=' => Ok(-2),
        '-' => Ok(-1),
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        _ => Err(SolveError::InvalidInput),
    }
}

fn to_char(snafu: i64) -> char {
    match snafu {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => unreachable!(),
    }
}

fn from_snafu(snafu: &str) -> Result<i64, SolveError> {
    let chars: Vec<_> = snafu.chars().map(|c| parse(c)).collect::<Result<_, _>>()?;

    Ok(chars
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, snafu)| acc + 5i64.pow(i as u32) * snafu))
}

fn to_snafu(mut number: i64) -> String {
    let highest_power = (0..).find(|&i| 5i64.pow(i as u32) > number).unwrap() + 1;

    let mut factors: Vec<_> = (0..highest_power)
        .rev()
        .map(|i| {
            let factor = (0..5)
                .filter(|factor| factor * 5i64.pow(i as u32) <= number)
                .max()
                .unwrap();
            number -= factor * 5i64.pow(i as u32);
            factor
        })
        .collect();

    for i in (0..factors.len()).rev() {
        match factors[i] {
            f @ 3..=5 => {
                factors[i - 1] += 1;
                factors[i] = f - 5;
            }
            _ => (),
        }
    }

    let start_index = factors.iter().take_while(|&f| *f == 0).count();
    String::from_iter(factors[start_index..].iter().map(|f| to_char(*f)))
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Vec<String>;
    type Output = String;
    const DAY: u8 = 25;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        Ok(input.lines().map(|line| line.to_owned()).collect())
    }

    fn part_1(input: Self::Input) -> Result<Self::Output, SolveError> {
        let numbers = input
            .iter()
            .map(|snafu| from_snafu(snafu))
            .collect::<Result<Vec<_>, _>>()?;

        let sum = numbers.into_iter().sum();
        Ok(to_snafu(sum))
    }
}

#[cfg(test)]
mod tests {
    use crate::solving::Solver;

    const SNAFU_TO_NUMBER: [(&str, i64); 13] = [
        ("1=-0-2", 1747),
        ("12111", 906),
        ("2=0=", 198),
        ("21", 11),
        ("2=01", 201),
        ("111", 31),
        ("20012", 1257),
        ("112", 32),
        ("1=-1=", 353),
        ("1-12", 107),
        ("12", 7),
        ("1=", 3),
        ("122", 37),
    ];

    fn get_input() -> Vec<String> {
        SNAFU_TO_NUMBER
            .iter()
            .map(|(snafu, _)| snafu.to_string())
            .collect()
    }

    #[test]
    fn from_snafu() {
        for (snafu, number) in SNAFU_TO_NUMBER {
            assert_eq!(super::from_snafu(snafu).unwrap(), number);
        }
    }

    #[test]
    fn to_snafu() {
        for (snafu, number) in SNAFU_TO_NUMBER {
            assert_eq!(&super::to_snafu(number), snafu);
        }
    }

    #[test]
    fn parsing() {
        let input = r"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

        assert_eq!(
            super::Solver::parse(String::from(input)).unwrap(),
            get_input()
        );
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(
            super::Solver::part_1(input).unwrap(),
            String::from("2=-1=0")
        );
    }
}
