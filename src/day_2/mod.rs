mod parsing;

use crate::{ParseError, SolveError};

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Vec<Round>;
    type Output = u64;
    const DAY: u8 = 2;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        parsing::parse(input)
    }

    fn part_1(input: Self::Input) -> Result<Self::Output, SolveError> {
        Ok(input.iter().fold(0, |acc, round| acc + round.get_score()))
    }

    fn part_2(input: Self::Input) -> Result<Self::Output, SolveError> {
        Ok(input
            .iter()
            .map(|round| round.to_strategy())
            .fold(0, |acc, round| acc + round.get_score()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl From<i8> for Shape {
    fn from(val: i8) -> Self {
        match val.rem_euclid(3) {
            0 => Shape::Rock,
            1 => Shape::Paper,
            2 => Shape::Scissors,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Round {
    pub opponent: Shape,
    pub player: Shape,
}

impl Round {
    pub fn get_score(&self) -> u64 {
        (self.player as i8 + 1 + self.round_outcome()) as u64
    }

    fn round_outcome(&self) -> i8 {
        3 * (self.player as i8 - self.opponent as i8 + 1).rem_euclid(3)
    }

    pub fn to_strategy(&self) -> Self {
        Self {
            opponent: self.opponent,
            player: match self.player {
                Shape::Rock => Shape::from(self.opponent as i8 - 1),
                Shape::Paper => self.opponent,
                Shape::Scissors => Shape::from(self.opponent as i8 + 1),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_vs_paper() {
        let round = Round {
            opponent: Shape::Rock,
            player: Shape::Paper,
        };
        assert_eq!(round.get_score(), 8);
    }

    #[test]
    fn paper_vs_rock() {
        let round = Round {
            opponent: Shape::Paper,
            player: Shape::Rock,
        };
        assert_eq!(round.get_score(), 1);
    }

    #[test]
    fn scissors_vs_scissors() {
        let round = Round {
            opponent: Shape::Scissors,
            player: Shape::Scissors,
        };
        assert_eq!(round.get_score(), 6);
    }

    #[test]
    fn draw_to_rock() {
        let round = Round {
            opponent: Shape::Rock,
            player: Shape::Paper,
        };
        assert_eq!(round.to_strategy().get_score(), 4);
    }

    #[test]
    fn lose_to_paper() {
        let round = Round {
            opponent: Shape::Paper,
            player: Shape::Rock,
        };
        assert_eq!(round.to_strategy().get_score(), 1);
    }

    #[test]
    fn win_from_scissors() {
        let round = Round {
            opponent: Shape::Scissors,
            player: Shape::Scissors,
        };
        assert_eq!(round.to_strategy().get_score(), 7);
    }
}
