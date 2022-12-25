use crate::day_17::point::Point;
use crate::{ParseError, SolveError};

#[derive(Debug, PartialEq)]
pub struct Elf {
    cur_pos: Point,
}

impl Elf {
    fn new(pos: Point) -> Self {
        Self { cur_pos: pos }
    }

    fn is_alone(&self, elves: &[Elf]) -> bool {
        !elves.iter().filter(|&other| other != self).any(|other| {
            other.cur_pos.x.abs_diff(self.cur_pos.x) <= 1
                && other.cur_pos.y.abs_diff(self.cur_pos.y) <= 1
        })
    }

    fn can_move_north(&self, elves: &[Elf]) -> bool {
        !elves.iter().any(|other| other.is_north_of(self))
    }

    fn can_move_south(&self, elves: &[Elf]) -> bool {
        !elves.iter().any(|other| other.is_south_of(self))
    }

    fn can_move_west(&self, elves: &[Elf]) -> bool {
        !elves.iter().any(|other| other.is_west_of(self))
    }

    fn can_move_east(&self, elves: &[Elf]) -> bool {
        !elves.iter().any(|other| other.is_east_of(self))
    }

    fn is_north_of(&self, other: &Elf) -> bool {
        self.cur_pos.y == other.cur_pos.y - 1 && self.cur_pos.x.abs_diff(other.cur_pos.x) <= 1
    }

    fn is_south_of(&self, other: &Elf) -> bool {
        self.cur_pos.y == other.cur_pos.y + 1 && self.cur_pos.x.abs_diff(other.cur_pos.x) <= 1
    }

    fn is_west_of(&self, other: &Elf) -> bool {
        self.cur_pos.x == other.cur_pos.x - 1 && self.cur_pos.y.abs_diff(other.cur_pos.y) <= 1
    }

    fn is_east_of(&self, other: &Elf) -> bool {
        self.cur_pos.x == other.cur_pos.x + 1 && self.cur_pos.y.abs_diff(other.cur_pos.y) <= 1
    }
}

fn get_rect(elves: &[Elf]) -> (Point, Point) {
    let x_min = elves.iter().map(|elf| elf.cur_pos.x).min().unwrap_or(0);
    let x_max = elves.iter().map(|elf| elf.cur_pos.x).max().unwrap_or(0);
    let y_min = elves.iter().map(|elf| elf.cur_pos.y).min().unwrap_or(0);
    let y_max = elves.iter().map(|elf| elf.cur_pos.y).max().unwrap_or(0);

    (Point { x: x_min, y: y_min }, Point { x: x_max, y: y_max })
}

fn count_empty_tiles(rect: &(Point, Point), elves: &[Elf]) -> usize {
    ((rect.1.x - rect.0.x + 1) * (rect.1.y - rect.0.y + 1)) as usize - elves.len()
}

#[allow(unused)]
fn print(elves: &[Elf]) {
    let rect = get_rect(elves);

    for y in rect.0.y..=rect.1.y {
        for x in rect.0.x..=rect.1.x {
            if elves.iter().any(|elf| elf.cur_pos == Point { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }

    println!();
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Vec<Elf>;
    type Output = usize;
    const DAY: u8 = 23;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        Ok(input
            .lines()
            .enumerate()
            .map(|(index, line)| parse_line(line, index))
            .flatten()
            .collect())
    }

    fn part_1(mut elves: Self::Input) -> Result<Self::Output, SolveError> {
        let strats: [Box<dyn Fn(&Elf, &[Elf]) -> Option<Point>>; 4] = [
            Box::new(|elf, elves| {
                if elf.can_move_north(elves) {
                    Some(Point::new(elf.cur_pos.x, elf.cur_pos.y - 1))
                } else {
                    None
                }
            }),
            Box::new(|elf, elves| {
                if elf.can_move_south(elves) {
                    Some(Point::new(elf.cur_pos.x, elf.cur_pos.y + 1))
                } else {
                    None
                }
            }),
            Box::new(|elf, elves| {
                if elf.can_move_west(elves) {
                    Some(Point::new(elf.cur_pos.x - 1, elf.cur_pos.y))
                } else {
                    None
                }
            }),
            Box::new(|elf, elves| {
                if elf.can_move_east(elves) {
                    Some(Point::new(elf.cur_pos.x + 1, elf.cur_pos.y))
                } else {
                    None
                }
            }),
        ];

        for round in 0..10 {
            let mut next_positions: Vec<_> = elves
                .iter()
                .map(|elf| {
                    if elf.is_alone(&elves) {
                        None
                    } else if let Some(pos) = strats[round % strats.len()](elf, &elves) {
                        Some(pos)
                    } else if let Some(pos) = strats[(round + 1) % strats.len()](elf, &elves) {
                        Some(pos)
                    } else if let Some(pos) = strats[(round + 2) % strats.len()](elf, &elves) {
                        Some(pos)
                    } else if let Some(pos) = strats[(round + 3) % strats.len()](elf, &elves) {
                        Some(pos)
                    } else {
                        None
                    }
                })
                .collect();

            let duplicates: Vec<_> = next_positions
                .iter()
                .map(|pos| next_positions.iter().filter(|&other| other == pos).count() > 1)
                .collect();

            for (pos, is_duplicate) in next_positions.iter_mut().zip(duplicates.iter()) {
                if *is_duplicate {
                    *pos = None;
                }
            }

            for (elf, pos) in elves.iter_mut().zip(next_positions.into_iter()) {
                if let Some(pos) = pos {
                    elf.cur_pos = pos;
                }
            }
        }

        let rect = get_rect(&elves);

        Ok(count_empty_tiles(&rect, &elves))
    }
}

fn parse_line(line: &str, index: usize) -> Vec<Elf> {
    line.char_indices()
        .flat_map(|(i, c)| {
            if c == '#' {
                Some(Elf::new(Point {
                    x: i as i64,
                    y: index as i64,
                }))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    use super::*;

    fn get_input() -> Vec<Elf> {
        vec![
            Elf::new(Point { x: 4, y: 0 }),
            Elf::new(Point { x: 2, y: 1 }),
            Elf::new(Point { x: 3, y: 1 }),
            Elf::new(Point { x: 4, y: 1 }),
            Elf::new(Point { x: 6, y: 1 }),
            Elf::new(Point { x: 0, y: 2 }),
            Elf::new(Point { x: 4, y: 2 }),
            Elf::new(Point { x: 6, y: 2 }),
            Elf::new(Point { x: 1, y: 3 }),
            Elf::new(Point { x: 5, y: 3 }),
            Elf::new(Point { x: 6, y: 3 }),
            Elf::new(Point { x: 0, y: 4 }),
            Elf::new(Point { x: 2, y: 4 }),
            Elf::new(Point { x: 3, y: 4 }),
            Elf::new(Point { x: 4, y: 4 }),
            Elf::new(Point { x: 0, y: 5 }),
            Elf::new(Point { x: 1, y: 5 }),
            Elf::new(Point { x: 3, y: 5 }),
            Elf::new(Point { x: 5, y: 5 }),
            Elf::new(Point { x: 6, y: 5 }),
            Elf::new(Point { x: 1, y: 6 }),
            Elf::new(Point { x: 4, y: 6 }),
        ]
    }

    #[test]
    fn parsing() {
        let input = r"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

        assert_eq!(
            super::Solver::parse(String::from(input)).unwrap(),
            get_input()
        )
    }

    #[test]
    fn part_1() {
        assert_eq!(super::Solver::part_1(get_input()).unwrap(), 110);
    }
}
