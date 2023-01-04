use std::convert::{TryFrom, TryInto};
use std::fmt::{self, Display, Formatter};

use crate::day_17::point::Point;
use crate::{ParseError, SolveError};

#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    width: usize,
    height: usize,
    destination: Point,
    blizzards: Vec<u8>,
}

pub enum Direction {
    North = 1,
    East = 2,
    South = 4,
    West = 8,
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Direction::North),
            2 => Ok(Direction::East),
            4 => Ok(Direction::South),
            8 => Ok(Direction::West),
            _ => Err(()),
        }
    }
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            destination: Point::new(0, 0),
            blizzards: [0].repeat(width * height),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.blizzards[self.get_index(x, y)]
    }

    pub fn get_num_blizzards(&self, x: usize, y: usize) -> u32 {
        self.get(x, y).count_ones()
    }

    pub fn is_valid(&self, pos: &Point) -> bool {
        let &Point { x, y } = pos;
        (x > 0 && x < (self.width - 1) as i64 && y > 0 && y < (self.height - 1) as i64)
            || pos == &self.destination
    }

    pub fn add_blizzard(&mut self, x: usize, y: usize, dir: Direction) {
        let index = self.get_index(x, y);
        self.blizzards[index] |= dir as u8;
    }

    pub fn has_blizzard(&self, x: usize, y: usize, dir: Direction) -> bool {
        self.get(x, y) & dir as u8 != 0
    }

    pub fn has_any_blizzard(&self, point: &Point) -> bool {
        let &Point { x, y } = point;
        self.get(x as usize, y as usize) > 0
    }

    pub fn remove_blizzard(&mut self, x: usize, y: usize, dir: Direction) {
        let index = self.get_index(x, y);
        self.blizzards[index] &= !(dir as u8);
    }

    pub fn updated(&self) -> Self {
        let mut new_grid = Grid {
            blizzards: [0].repeat(self.width * self.height),
            ..*self
        };

        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                if self.has_blizzard(x, y, Direction::North) {
                    new_grid.add_blizzard(x, y - 1, Direction::North);
                }

                if self.has_blizzard(x, y, Direction::East) {
                    new_grid.add_blizzard(x + 1, y, Direction::East);
                }

                if self.has_blizzard(x, y, Direction::South) {
                    new_grid.add_blizzard(x, y + 1, Direction::South);
                }

                if self.has_blizzard(x, y, Direction::West) {
                    new_grid.add_blizzard(x - 1, y, Direction::West);
                }
            }
        }

        for x in 0..new_grid.width {
            if new_grid.has_blizzard(x, 0, Direction::North) {
                new_grid.remove_blizzard(x, 0, Direction::North);
                new_grid.add_blizzard(x, new_grid.height - 2, Direction::North);
            }

            if new_grid.has_blizzard(x, new_grid.height - 1, Direction::South) {
                new_grid.remove_blizzard(x, new_grid.height - 1, Direction::South);
                new_grid.add_blizzard(x, 1, Direction::South);
            }
        }

        for y in 0..new_grid.height {
            if new_grid.has_blizzard(0, y, Direction::West) {
                new_grid.remove_blizzard(0, y, Direction::West);
                new_grid.add_blizzard(new_grid.width - 2, y, Direction::West);
            }

            if new_grid.has_blizzard(new_grid.width - 1, y, Direction::East) {
                new_grid.remove_blizzard(new_grid.width - 1, y, Direction::East);
                new_grid.add_blizzard(1, y, Direction::East);
            }
        }

        new_grid
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let _ = writeln!(f, "Grid {{")?;

        for y in 0..self.height {
            let _ = write!(f, "    ")?;

            for x in 0..self.width {
                let _ = match self.get_num_blizzards(x, y) {
                    0 => write!(f, "."),
                    1 => match self.get(x, y).try_into() {
                        Ok(Direction::North) => write!(f, "^"),
                        Ok(Direction::East) => write!(f, ">"),
                        Ok(Direction::South) => write!(f, "v"),
                        Ok(Direction::West) => write!(f, "<"),
                        Err(_) => unreachable!(),
                    },
                    num_blizzards => write!(f, "{num_blizzards}"),
                }?;
            }

            let _ = writeln!(f)?;
        }

        let _ = writeln!(f, "}}")?;

        Ok(())
    }
}

fn generate_cyclic_grids(initial_grid: Grid) -> Vec<Grid> {
    // TODO: Use lcm(width, height)
    let period = initial_grid.width * initial_grid.height;

    let mut grids = vec![initial_grid];

    for _ in 0..period {
        let new_grid = grids.last().unwrap().updated();
        grids.push(new_grid);
    }

    grids
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = (Grid, Point);
    type Output = usize;
    const DAY: u8 = 24;

    fn parse(_input: String) -> Result<Self::Input, ParseError> {
        todo!()
    }

    fn part_1(input: Self::Input) -> Result<Self::Output, SolveError> {
        let (grid, expedition) = input;

        let grids = generate_cyclic_grids(grid);

        get_fastest_path(&grids, 1, expedition + Point::new(0, 1), usize::MAX)
            .ok_or(SolveError::InvalidInput)
    }
}

fn get_fastest_path(
    grids: &[Grid],
    round: usize,
    expedition: Point,
    fastest_path: usize,
) -> Option<usize> {
    if round >= fastest_path || round > 20 {
        return None;
    }

    let grid = &grids[round % grids.len()];

    if expedition + Point::new(0, 1) == grid.destination {
        return Some(round);
    }

    let new_path_south = if can_move_south(grid, expedition) {
        get_fastest_path(
            grids,
            round + 1,
            expedition + Point::new(0, 1),
            fastest_path,
        )
    } else {
        None
    };

    let new_path_east = if can_move_east(grid, expedition) {
        get_fastest_path(
            grids,
            round + 1,
            expedition + Point::new(1, 0),
            fastest_path,
        )
    } else {
        None
    };

    let new_path_west = if can_move_west(grid, expedition) {
        get_fastest_path(
            grids,
            round + 1,
            expedition + Point::new(-1, 0),
            fastest_path,
        )
    } else {
        None
    };

    let new_path_north = if can_move_north(grid, expedition) {
        get_fastest_path(
            grids,
            round + 1,
            expedition + Point::new(0, -1),
            fastest_path,
        )
    } else {
        None
    };

    let new_path_stationary = if can_stay_stationary(grid, expedition) {
        get_fastest_path(grids, round + 1, expedition, fastest_path)
    } else {
        None
    };

    let new_paths = [
        new_path_north,
        new_path_east,
        new_path_south,
        new_path_west,
        new_path_stationary,
    ];
    new_paths.iter().filter_map(|new_path| *new_path).min()
}

fn can_move_north(grid: &Grid, expedition: Point) -> bool {
    let new_pos = expedition + Point::new(0, -1);

    grid.is_valid(&new_pos) && !grid.has_any_blizzard(&new_pos)
}

fn can_move_east(grid: &Grid, expedition: Point) -> bool {
    let new_pos = expedition + Point::new(1, 0);

    grid.is_valid(&new_pos) && !grid.has_any_blizzard(&new_pos)
}

fn can_move_south(grid: &Grid, expedition: Point) -> bool {
    let new_pos = expedition + Point::new(0, 1);

    grid.is_valid(&new_pos) && !grid.has_any_blizzard(&new_pos)
}

fn can_move_west(grid: &Grid, expedition: Point) -> bool {
    let new_pos = expedition + Point::new(-1, 0);

    grid.is_valid(&new_pos) && !grid.has_any_blizzard(&new_pos)
}

fn can_stay_stationary(grid: &Grid, expedition: Point) -> bool {
    !grid.has_any_blizzard(&expedition)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Solver;

    fn get_input() -> (Grid, Point) {
        let mut grid = Grid::new(8, 6);

        grid.add_blizzard(1, 1, Direction::East);
        grid.add_blizzard(2, 1, Direction::East);
        grid.add_blizzard(4, 1, Direction::West);
        grid.add_blizzard(5, 1, Direction::North);
        grid.add_blizzard(6, 1, Direction::West);

        grid.add_blizzard(2, 2, Direction::West);
        grid.add_blizzard(5, 2, Direction::West);
        grid.add_blizzard(6, 2, Direction::West);

        grid.add_blizzard(1, 3, Direction::East);
        grid.add_blizzard(2, 3, Direction::South);
        grid.add_blizzard(4, 3, Direction::East);
        grid.add_blizzard(5, 3, Direction::West);
        grid.add_blizzard(6, 3, Direction::East);

        grid.add_blizzard(1, 4, Direction::West);
        grid.add_blizzard(2, 4, Direction::North);
        grid.add_blizzard(3, 4, Direction::South);
        grid.add_blizzard(4, 4, Direction::North);
        grid.add_blizzard(5, 4, Direction::North);
        grid.add_blizzard(6, 4, Direction::East);

        grid.destination = Point::new(6, 5);

        (grid, Point::new(1, 0))
    }

    #[test]
    fn parsing() {
        let input = r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

        assert_eq!(
            super::Solver::parse(String::from(input)).unwrap(),
            get_input(),
        );
    }

    #[test]
    fn update_grid() {
        let (mut grid, _) = get_input();

        for _ in 0..18 {
            grid = grid.updated();
        }

        let mut ref_grid = Grid::new(8, 6);

        ref_grid.add_blizzard(1, 1, Direction::East);
        ref_grid.add_blizzard(2, 1, Direction::East);
        ref_grid.add_blizzard(2, 1, Direction::South);
        ref_grid.add_blizzard(4, 1, Direction::West);
        ref_grid.add_blizzard(6, 1, Direction::West);

        ref_grid.add_blizzard(2, 2, Direction::West);
        ref_grid.add_blizzard(2, 2, Direction::North);
        ref_grid.add_blizzard(3, 2, Direction::South);
        ref_grid.add_blizzard(4, 2, Direction::North);
        ref_grid.add_blizzard(5, 2, Direction::West);
        ref_grid.add_blizzard(5, 2, Direction::North);
        ref_grid.add_blizzard(6, 2, Direction::West);

        ref_grid.add_blizzard(1, 3, Direction::East);
        ref_grid.add_blizzard(4, 3, Direction::East);
        ref_grid.add_blizzard(5, 3, Direction::North);
        ref_grid.add_blizzard(5, 3, Direction::West);
        ref_grid.add_blizzard(6, 3, Direction::East);

        ref_grid.add_blizzard(1, 4, Direction::West);
        ref_grid.add_blizzard(6, 4, Direction::East);

        assert_eq!(grid, ref_grid);
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(super::Solver::part_1(input).unwrap(), 18);
    }
}
