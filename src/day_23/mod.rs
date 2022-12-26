use crate::day_17::point::Point;
use crate::{ParseError, SolveError};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Elf {
    cur_pos: Point,
}

impl Elf {
    fn new(pos: Point) -> Self {
        Self { cur_pos: pos }
    }

    fn is_alone(&self, elves: &[Elf]) -> bool {
        let adj_points: [Point; 8] = [
            self.cur_pos + Point { x: -1, y: -1 },
            self.cur_pos + Point { x: 0, y: -1 },
            self.cur_pos + Point { x: 1, y: -1 },
            self.cur_pos + Point { x: -1, y: 0 },
            self.cur_pos + Point { x: 1, y: 0 },
            self.cur_pos + Point { x: -1, y: 1 },
            self.cur_pos + Point { x: 0, y: 1 },
            self.cur_pos + Point { x: 1, y: 1 },
        ];

        !elves
            .iter()
            .any(|other| adj_points.iter().any(|p| &other.cur_pos == p))
    }

    fn try_propose_north(&self, elves: &[Elf]) -> Option<Point> {
        if !elves.iter().any(|other| {
            other.cur_pos.y == self.cur_pos.y - 1 && other.cur_pos.x.abs_diff(self.cur_pos.x) <= 1
        }) {
            Some(self.cur_pos + Point { x: 0, y: -1 })
        } else {
            None
        }
    }

    fn try_propose_south(&self, elves: &[Elf]) -> Option<Point> {
        if !elves.iter().any(|other| {
            other.cur_pos.y == self.cur_pos.y + 1 && other.cur_pos.x.abs_diff(self.cur_pos.x) <= 1
        }) {
            Some(self.cur_pos + Point { x: 0, y: 1 })
        } else {
            None
        }
    }

    fn try_propose_west(&self, elves: &[Elf]) -> Option<Point> {
        if !elves.iter().any(|other| {
            other.cur_pos.x == self.cur_pos.x - 1 && other.cur_pos.y.abs_diff(self.cur_pos.y) <= 1
        }) {
            Some(self.cur_pos + Point { x: -1, y: 0 })
        } else {
            None
        }
    }

    fn try_propose_east(&self, elves: &[Elf]) -> Option<Point> {
        if !elves.iter().any(|other| {
            other.cur_pos.x == self.cur_pos.x + 1 && other.cur_pos.y.abs_diff(self.cur_pos.y) <= 1
        }) {
            Some(self.cur_pos + Point { x: 1, y: 0 })
        } else {
            None
        }
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

type Strategies = [Box<dyn Fn(&Elf, &[Elf]) -> Option<Point>>; 4];

fn get_strategies() -> Strategies {
    [
        Box::new(Elf::try_propose_north),
        Box::new(Elf::try_propose_south),
        Box::new(Elf::try_propose_west),
        Box::new(Elf::try_propose_east),
    ]
}

fn move_elves(elves: &mut [Elf], round: usize, strats: &Strategies) -> bool {
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

    for (pos, is_duplicate) in next_positions.iter_mut().zip(duplicates.into_iter()) {
        if is_duplicate {
            *pos = None;
        }
    }

    let any_elf_moves = next_positions.iter().any(|p| p.is_some());

    for (elf, pos) in elves.iter_mut().zip(next_positions.into_iter()) {
        if let Some(pos) = pos {
            elf.cur_pos = pos;
        }
    }

    any_elf_moves
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
        let strats = get_strategies();

        for round in 0..10 {
            let _ = move_elves(&mut elves, round, &strats);
        }

        let rect = get_rect(&elves);

        Ok(count_empty_tiles(&rect, &elves))
    }

    fn part_2(mut elves: Self::Input) -> Result<Self::Output, SolveError> {
        let strats = get_strategies();

        for round in 0.. {
            let b = move_elves(&mut elves, round, &strats);

            if !b {
                return Ok(round + 1);
            }
        }

        unreachable!()
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

    #[test]
    fn part_2() {
        assert_eq!(super::Solver::part_2(get_input()).unwrap(), 20);
    }
}
