use crate::ParseError;

use super::{Round, Shape, Solver};

pub fn parse(input: String) -> Result<<Solver as crate::Solver>::Input, ParseError> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Result<Round, ParseError> {
    let mut chars = line.chars();

    let opponent = chars.nth(0).ok_or(ParseError::Incomplete)?;
    let opponent = parse_opponent_shape(opponent)?;

    let player = chars.nth(1).ok_or(ParseError::Incomplete)?;
    let player = parse_player_shape(player)?;

    Ok(Round { opponent, player })
}

fn parse_opponent_shape(c: char) -> Result<Shape, ParseError> {
    match c {
        'A' => Ok(Shape::Rock),
        'B' => Ok(Shape::Paper),
        'C' => Ok(Shape::Scissors),
        _ => Err(ParseError::Invalid),
    }
}

fn parse_player_shape(c: char) -> Result<Shape, ParseError> {
    match c {
        'X' => Ok(Shape::Rock),
        'Y' => Ok(Shape::Paper),
        'Z' => Ok(Shape::Scissors),
        _ => Err(ParseError::Invalid),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = r"A Y
B X
C Z";

        let input = parse(String::from(input));
        let ref_input = vec![
            Round {
                opponent: Shape::Rock,
                player: Shape::Paper,
            },
            Round {
                opponent: Shape::Paper,
                player: Shape::Rock,
            },
            Round {
                opponent: Shape::Scissors,
                player: Shape::Scissors,
            },
        ];

        assert_eq!(input.unwrap(), ref_input);
    }
}
