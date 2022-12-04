mod parsing;

use std::cmp::Ordering;

use crate::{ParseError, SolveResult};

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Vec<Round>;
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn cmp(&self, other: &Shape) -> Ordering {
        match self {
            Shape::Rock => match other {
                Shape::Rock => Ordering::Equal,
                Shape::Paper => Ordering::Less,
                Shape::Scissors => Ordering::Greater,
            },
            Shape::Paper => match other {
                Shape::Rock => Ordering::Greater,
                Shape::Paper => Ordering::Equal,
                Shape::Scissors => Ordering::Less,
            },
            Shape::Scissors => match other {
                Shape::Rock => Ordering::Less,
                Shape::Paper => Ordering::Greater,
                Shape::Scissors => Ordering::Equal,
            },
        }
    }

    pub fn get_value(&self) -> u64 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
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
        self.player.get_value()
            + match self.player.cmp(&self.opponent) {
                Ordering::Less => 0,
                Ordering::Equal => 3,
                Ordering::Greater => 6,
            }
    }

    pub fn to_strategy(&self) -> Self {
        let player = match self.player {
            // Lose
            Shape::Rock => match self.opponent {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            // Draw
            Shape::Paper => self.opponent,
            // Win
            Shape::Scissors => match self.opponent {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
        };

        Self {
            opponent: self.opponent,
            player,
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
