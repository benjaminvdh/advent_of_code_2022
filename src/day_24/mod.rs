use std::convert::{TryFrom, TryInto};

use crate::{ParseError, SolveError};

#[derive(Debug, PartialEq)]
pub struct Grid {
    width: usize,
    height: usize,
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
            blizzards: [0].repeat(width * height),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.blizzards[self.get_index(x, y)]
    }

    pub fn get_num_blizzards(&self, x: usize, y: usize) -> u32 {
        self.get(x, y).count_ones()
    }

    pub fn add_blizzard(&mut self, x: usize, y: usize, dir: Direction) {
        let index = self.get_index(x, y);
        self.blizzards[index] |= dir as u8;
    }

    pub fn has_blizzard(&self, x: usize, y: usize, dir: Direction) -> bool {
        self.get(x, y) & dir as u8 != 0
    }

    pub fn remove_blizzard(&mut self, x: usize, y: usize, dir: Direction) {
        let index = self.get_index(x, y);
        self.blizzards[index] &= !(dir as u8);
    }

    pub fn updated(&self) -> Self {
        let mut new_grid = Self::new(self.width, self.height);

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

    #[allow(unused)]
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_num_blizzards(x, y) {
                    0 => print!("."),
                    1 => match self.get(x, y).try_into() {
                        Ok(Direction::North) => print!("^"),
                        Ok(Direction::East) => print!(">"),
                        Ok(Direction::South) => print!("v"),
                        Ok(Direction::West) => print!("<"),
                        Err(_) => unreachable!(),
                    },
                    num_blizzards => print!("{num_blizzards}"),
                }
            }

            println!();
        }

        println!();
    }
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Grid;
    type Output = usize;
    const DAY: u8 = 24;

    fn parse(_input: String) -> Result<Self::Input, ParseError> {
        todo!()
    }

    fn part_1(_input: Self::Input) -> Result<Self::Output, SolveError> {
        Err(SolveError::Unimplemented)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Solver;

    fn get_input() -> Grid {
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

        grid
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
        let mut grid = get_input();

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
}
