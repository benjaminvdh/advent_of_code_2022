use crate::{ParseError, SolveError};

mod chamber;
pub mod point;
mod rock;

use chamber::Chamber;
use point::Point;
use rock::{Rock, PATTERNS};

#[allow(unused)]
#[cfg(not(test))]
fn print_state(chamber: &Chamber, rock: Option<&Rock>) {
    for y in (1..=chamber.get_height() + 6).rev() {
        print!("|");

        for x in 0..Chamber::WIDTH {
            if rock.is_some()
                && rock
                    .unwrap()
                    .get_points()
                    .find(|p| p.x == x && p.y == y)
                    .is_some()
            {
                print!("@");
            } else if chamber.is_blocked(&Point::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!("|");
    }

    println!("+-------+");

    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}

#[allow(unused)]
#[cfg(test)]
fn print_state(_: &Chamber, _: Option<&Rock>) {}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Vec<Point>;
    type Output = i64;
    const DAY: u8 = 17;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        Ok(input
            .chars()
            .filter_map(|c| match c {
                '<' => Some(Point::LEFT),
                '>' => Some(Point::RIGHT),
                _ => None,
            })
            .collect())
    }

    fn part_1(input: Self::Input) -> Result<Self::Output, SolveError> {
        let mut chamber = Chamber::default();

        let mut movement_iter = input.iter().cycle();

        for pattern in PATTERNS.iter().cycle().take(2022) {
            let mut rock = chamber.spawn_rock(pattern);

            loop {
                let movement = movement_iter.next().unwrap();

                if chamber.can_move(&rock, &movement) {
                    rock.position += movement;
                }

                if chamber.can_move(&rock, &Point::DOWN) {
                    rock.position += Point::DOWN;
                } else {
                    chamber.place_rock(&rock);
                    break;
                }
            }
        }

        Ok(chamber.get_height())
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    use super::Point;

    fn get_input() -> Vec<Point> {
        vec![
            Point::RIGHT,
            Point::RIGHT,
            Point::RIGHT,
            Point::LEFT,
            Point::LEFT,
            Point::RIGHT,
            Point::LEFT,
            Point::RIGHT,
            Point::RIGHT,
            Point::LEFT,
            Point::LEFT,
            Point::LEFT,
            Point::RIGHT,
            Point::RIGHT,
            Point::LEFT,
            Point::RIGHT,
            Point::RIGHT,
            Point::RIGHT,
            Point::LEFT,
            Point::LEFT,
            Point::LEFT,
            Point::RIGHT,
            Point::RIGHT,
            Point::RIGHT,
            Point::LEFT,
            Point::LEFT,
            Point::LEFT,
            Point::RIGHT,
            Point::LEFT,
            Point::LEFT,
            Point::LEFT,
            Point::RIGHT,
            Point::RIGHT,
            Point::LEFT,
            Point::RIGHT,
            Point::RIGHT,
            Point::LEFT,
            Point::LEFT,
            Point::RIGHT,
            Point::RIGHT,
        ]
    }

    #[test]
    fn parsing() {
        let input = r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        assert_eq!(
            super::Solver::parse(String::from(input)).unwrap(),
            get_input()
        );
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(super::Solver::part_1(input).unwrap(), 3068);
    }
}
