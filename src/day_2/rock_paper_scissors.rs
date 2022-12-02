use std::cmp::Ordering;

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
        self.player.get_value() + match self.player.cmp(&self.opponent) {
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

        Self { opponent: self.opponent, player }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod as_shape {
        use super::*;

        #[test]
        fn rock_vs_paper() {
            let round = Round { opponent: Shape::Rock, player: Shape::Paper };
            assert_eq!(round.get_score(), 8);
        }

        #[test]
        fn paper_vs_rock() {
            let round = Round { opponent: Shape::Paper, player: Shape::Rock };
            assert_eq!(round.get_score(), 1);
        }

        #[test]
        fn scissors_vs_scissors() {
            let round = Round { opponent: Shape::Scissors, player: Shape::Scissors };
            assert_eq!(round.get_score(), 6);
        }
    }

    mod as_strategy {
        use super::*;

        #[test]
        fn rock_vs_paper() {
            let round = Round { opponent: Shape::Rock, player: Shape::Paper };
            assert_eq!(round.to_strategy().get_score(), 4);
        }

        #[test]
        fn paper_vs_rock() {
            let round = Round { opponent: Shape::Paper, player: Shape::Rock };
            assert_eq!(round.to_strategy().get_score(), 1);
        }

        #[test]
        fn scissors_vs_scissors() {
            let round = Round { opponent: Shape::Scissors, player: Shape::Scissors };
            assert_eq!(round.to_strategy().get_score(), 7);
        }
    }
}
