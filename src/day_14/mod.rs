use std::collections::BTreeSet;
use std::fmt::{self, Debug, Formatter};

use crate::{ParseError, SolveError};

#[derive(PartialEq)]
pub struct Grid {
    cells: Vec<bool>,
    min_x: usize,
    min_y: usize,
    width: usize,
    height: usize,
    overflow: BTreeSet<(usize, usize)>,
}

impl Grid {
    fn new(min_x: usize, max_x: usize, max_y: usize) -> Self {
        let width = (max_x - min_x + 3) as usize;
        let height = (max_y + 2) as usize;

        Self {
            cells: [false].repeat(width * height),
            min_x: min_x - 1,
            min_y: 0,
            width,
            height,
            overflow: BTreeSet::new(),
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        if self.is_valid_index(x, y) {
            self.cells[x - self.min_x + (y - self.min_y) * self.width]
        } else {
            self.overflow.contains(&(x, y))
        }
    }

    fn set(&mut self, x: usize, y: usize) {
        if self.is_valid_index(x, y) {
            self.cells[x - self.min_x + (y - self.min_y) * self.width] = true;
        } else {
            let _ = self.overflow.insert((x, y));
        }
    }

    fn is_valid_index(&self, x: usize, y: usize) -> bool {
        self.min_x <= x && x < self.min_x + self.width && y < self.height
    }

    fn add_line(&mut self, line: &[(usize, usize)]) {
        for line in line.windows(2) {
            let (start_x, start_y) = line[0];
            let (end_x, end_y) = line[1];

            if start_x == end_x {
                for y in start_y.min(end_y)..=start_y.max(end_y) {
                    self.set(start_x, y);
                }
            } else if start_y == end_y {
                for x in start_x.min(end_x)..=start_x.max(end_x) {
                    self.set(x, start_y);
                }
            }
        }
    }

    fn add_sand(&mut self) -> Option<(usize, usize)> {
        let sand = self.drop_sand(500, 0);

        if let Some((x, y)) = sand {
            self.set(x, y);
        }

        sand
    }

    fn drop_sand(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
        for y in y..self.height {
            if self.get(x, y) {
                if !self.get(x - 1, y) {
                    return self.drop_sand(x - 1, y);
                } else if !self.get(x + 1, y) {
                    return self.drop_sand(x + 1, y);
                } else {
                    return Some((x, y - 1));
                }
            }
        }

        Some((x, self.height - 1))
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let _ = writeln!(
            f,
            "Grid {{ min_x: {}, min_y: {}, width: {}, height: {}, cells:",
            self.min_x, self.min_y, self.width, self.height
        )?;

        for y in self.min_y..self.min_y + self.height + 6 {
            for x in self.min_x - 20..self.min_x + self.width + 20 {
                if self.get(x, y) {
                    let _ = write!(f, "#")?;
                } else {
                    let _ = write!(f, ".")?;
                }
            }

            let _ = write!(f, "\n")?;
        }

        write!(f, "}}")
    }
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Grid;
    type Output = usize;
    const DAY: u8 = 14;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        let lines = input
            .lines()
            .map(|line| parse_line(line))
            .collect::<Result<Vec<_>, _>>()?;

        let min_x = get_min(&lines, |(x, _)| *x)?;
        let max_x = get_max(&lines, |(x, _)| *x)?;
        let max_y = get_max(&lines, |(_, y)| *y)?;

        let mut grid = Grid::new(min_x, max_x, max_y);

        for line in lines.iter() {
            grid.add_line(line);
        }

        Ok(grid)
    }

    fn part_1(mut grid: Self::Input) -> Result<Self::Output, SolveError> {
        let mut grains_of_sand = 0;

        loop {
            match grid.add_sand() {
                Some((_, y)) if y < grid.height - 1 => grains_of_sand += 1,
                _ => break,
            }
        }

        Ok(grains_of_sand)
    }

    fn part_2(mut grid: Self::Input) -> Result<Self::Output, SolveError> {
        let mut grains_of_sand = 0;

        loop {
            match grid.add_sand() {
                Some((500, 0)) => break,
                Some(_) => grains_of_sand += 1,
                None => return Err(SolveError::InvalidInput),
            }
        }

        Ok(grains_of_sand + 1)
    }
}

fn get_min<F>(coords: &Vec<Vec<(usize, usize)>>, chooser: F) -> Result<usize, ParseError>
where
    F: Fn(&(usize, usize)) -> usize,
{
    coords
        .iter()
        .flat_map(|coords| coords.iter())
        .map(chooser)
        .min()
        .ok_or(ParseError::Incomplete)
}

fn get_max<F>(coords: &Vec<Vec<(usize, usize)>>, chooser: F) -> Result<usize, ParseError>
where
    F: Fn(&(usize, usize)) -> usize,
{
    coords
        .iter()
        .flat_map(|coords| coords.iter())
        .map(chooser)
        .max()
        .ok_or(ParseError::Incomplete)
}

fn parse_line(line: &str) -> Result<Vec<(usize, usize)>, ParseError> {
    line.split(" -> ")
        .map(|coord| parse_coord(coord))
        .collect::<Result<_, _>>()
}

fn parse_coord(coord: &str) -> Result<(usize, usize), ParseError> {
    match coord.split_once(",") {
        Some((x, y)) => Ok((x.parse()?, y.parse()?)),
        None => Err(ParseError::Invalid),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::Solver;

    use super::Grid;

    fn get_input() -> Grid {
        let width = 12;
        let height = 11;

        let mut grid = Grid {
            cells: [false].repeat(width * height),
            min_x: 493,
            min_y: 0,
            width,
            height,
            overflow: BTreeSet::new(),
        };

        for x in 496..=498 {
            grid.set(x, 6);
        }

        for y in 4..=5 {
            grid.set(498, y);
        }

        for x in 494..=502 {
            grid.set(x, 9);
        }

        for y in 4..=9 {
            grid.set(502, y);
        }

        grid.set(503, 4);

        grid
    }

    #[test]
    fn parsing() {
        let input = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        assert_eq!(
            super::Solver::parse(String::from(input)).unwrap(),
            get_input()
        );
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(super::Solver::part_1(input).unwrap(), 24);
    }

    #[test]
    fn part_2() {
        let input = get_input();

        assert_eq!(super::Solver::part_2(input).unwrap(), 93);
    }

    #[test]
    fn empty() {
        let width = 12;
        let height = 11;

        let grid = Grid {
            cells: [false].repeat(width * height),
            min_x: 493,
            min_y: 0,
            width,
            height,
            overflow: BTreeSet::new(),
        };

        assert_eq!(super::Solver::part_1(grid).unwrap(), 0);
    }
}
