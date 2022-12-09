use crate::{ParseError, SolveError};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Point(i32, i32);

impl Point {
    fn update(&mut self, dir: &Direction) {
        match dir {
            Direction::Right => self.0 += 1,
            Direction::Left => self.0 -= 1,
            Direction::Up => self.1 += 1,
            Direction::Down => self.1 -= 1,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, PartialEq)]
pub struct Motion {
    direction: Direction,
    distance: u8,
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Vec<Motion>;
    type Output = usize;
    const DAY: u8 = 9;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        input.lines().map(|line| parse_line(line)).collect()
    }

    fn part_1(input: Self::Input) -> Result<Self::Output, SolveError> {
        let mut head = Point(0, 0);
        let mut tails = vec![Point(0, 0)];

        for motion in input {
            for _ in 0..motion.distance {
                head.update(&motion.direction);
                tails.push(move_towards(&head, tails.last().unwrap()));
            }
        }

        tails.sort();
        tails.dedup();

        Ok(tails.len())
    }

    fn part_2(input: Self::Input) -> Result<Self::Output, SolveError> {
        let mut knots = [Point(0, 0); 10];
        let mut tails = vec![Point(0, 0)];

        for motion in input {
            for _ in 0..motion.distance {
                knots[0].update(&motion.direction);

                for i in 1..knots.len() {
                    knots[i] = move_towards(&knots[i - 1], &knots[i]);
                }

                tails.push(knots[9]);
            }
        }

        tails.sort();
        tails.dedup();

        Ok(tails.len())
    }
}

fn parse_line(line: &str) -> Result<Motion, ParseError> {
    let direction = match line.chars().nth(0) {
        Some('R') => Ok(Direction::Right),
        Some('L') => Ok(Direction::Left),
        Some('U') => Ok(Direction::Up),
        Some('D') => Ok(Direction::Down),
        _ => Err(ParseError::Invalid),
    }?;

    let distance = line[2..].parse()?;

    Ok(Motion {
        direction,
        distance,
    })
}

fn move_towards(head: &Point, tail: &Point) -> Point {
    if head.0.abs_diff(tail.0) <= 1 && head.1.abs_diff(tail.1) <= 1 {
        *tail
    } else {
        Point(
            tail.0 + (head.0 - tail.0).signum(),
            tail.1 + (head.1 - tail.1).signum(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Solver;

    fn get_input() -> Vec<Motion> {
        vec![
            Motion {
                direction: Direction::Right,
                distance: 4,
            },
            Motion {
                direction: Direction::Up,
                distance: 4,
            },
            Motion {
                direction: Direction::Left,
                distance: 3,
            },
            Motion {
                direction: Direction::Down,
                distance: 1,
            },
            Motion {
                direction: Direction::Right,
                distance: 4,
            },
            Motion {
                direction: Direction::Down,
                distance: 1,
            },
            Motion {
                direction: Direction::Left,
                distance: 5,
            },
            Motion {
                direction: Direction::Right,
                distance: 2,
            },
        ]
    }

    #[test]
    fn parsing() {
        let input = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(
            super::Solver::parse(String::from(input)).unwrap(),
            get_input()
        );
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(super::Solver::part_1(input).unwrap(), 13);
    }

    #[test]
    fn part_2() {
        let input = vec![
            Motion {
                direction: Direction::Right,
                distance: 5,
            },
            Motion {
                direction: Direction::Up,
                distance: 8,
            },
            Motion {
                direction: Direction::Left,
                distance: 8,
            },
            Motion {
                direction: Direction::Down,
                distance: 3,
            },
            Motion {
                direction: Direction::Right,
                distance: 17,
            },
            Motion {
                direction: Direction::Down,
                distance: 10,
            },
            Motion {
                direction: Direction::Left,
                distance: 25,
            },
            Motion {
                direction: Direction::Up,
                distance: 20,
            },
        ];

        assert_eq!(super::Solver::part_2(input).unwrap(), 36);
    }
}
