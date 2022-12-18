use std::fmt::{self, Display, Formatter};

use crate::{ParseError, SolveError};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    pub fn manhattan_distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }

    pub fn get_neighbors(&self) -> impl Iterator<Item = Point> {
        vec![
            Point {
                x: self.x - 1,
                ..*self
            },
            Point {
                x: self.x + 1,
                ..*self
            },
            Point {
                y: self.y - 1,
                ..*self
            },
            Point {
                y: self.y + 1,
                ..*self
            },
            Point {
                z: self.z - 1,
                ..*self
            },
            Point {
                z: self.z + 1,
                ..*self
            },
        ]
        .into_iter()
    }
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Vec<Point>;
    type Output = usize;
    const DAY: u8 = 18;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        Ok(input
            .lines()
            .map(|line| parse_line(line))
            .collect::<Result<_, _>>()?)
    }

    fn part_1(input: Self::Input) -> Result<Self::Output, SolveError> {
        Ok(count_free_sides(&input))
    }

    fn part_2(input: Self::Input) -> Result<Self::Output, SolveError> {
        let mut grid = Grid::new(&input);

        grid.mark_edges_as_free();
        grid.mark_lava(&input);

        loop {
            let has_changed = grid.propagate_free_cubes();

            if !has_changed {
                break;
            }
        }

        Ok(count_free_sides(&input) - count_sides_touching_air_pockets(&grid))
    }
}

fn parse_line(line: &str) -> Result<Point, ParseError> {
    let mut splits = line.splitn(3, ',');

    let x = splits.next().ok_or(ParseError::Invalid)?.parse()?;
    let y = splits.next().ok_or(ParseError::Invalid)?.parse()?;
    let z = splits.next().ok_or(ParseError::Invalid)?.parse()?;

    Ok(Point { x, y, z })
}

fn count_free_sides(lava_cubes: &[Point]) -> usize {
    lava_cubes.len() * 6
        - lava_cubes
            .iter()
            .map(|point| {
                lava_cubes
                    .iter()
                    .filter(|other| point.manhattan_distance(other) == 1)
                    .count()
            })
            .sum::<usize>()
}

fn count_sides_touching_air_pockets(grid: &Grid) -> usize {
    let mut count = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            for z in 0..grid.depth {
                let point = Point { x, y, z };

                if matches!(grid.get(&point), State::Enclosed) {
                    count += point
                        .get_neighbors()
                        .filter(|neighbor| matches!(grid.get(neighbor), State::Lava))
                        .count();
                }
            }
        }
    }

    count
}

#[derive(Clone, Copy)]
enum State {
    Enclosed,
    Free,
    Lava,
}

struct Grid {
    width: usize,
    height: usize,
    depth: usize,
    grid: Vec<State>,
}

impl Grid {
    fn new(cubes: &[Point]) -> Self {
        let width = cubes.iter().map(|point| point.x).max().unwrap_or(0) + 1;
        let height = cubes.iter().map(|point| point.y).max().unwrap_or(0) + 1;
        let depth = cubes.iter().map(|point| point.z).max().unwrap_or(0) + 1;

        Self {
            width,
            height,
            depth,
            grid: [State::Enclosed].repeat(width * height * depth),
        }
    }

    fn mark_edges_as_free(&mut self) {
        let x_max = self.width - 1;
        let y_max = self.height - 1;
        let z_max = self.depth - 1;

        for x in 0..self.width {
            for y in 0..self.height {
                self.set(&Point { x, y, z: 0 }, State::Free);
                self.set(&Point { x, y, z: z_max }, State::Free);
            }
        }

        for x in 0..self.width {
            for z in 0..self.depth {
                self.set(&Point { x, y: 0, z }, State::Free);
                self.set(&Point { x, y: y_max, z }, State::Free);
            }
        }

        for y in 0..self.height {
            for z in 0..self.depth {
                self.set(&Point { x: 0, y, z }, State::Free);
                self.set(&Point { x: x_max, y, z }, State::Free);
            }
        }
    }

    fn mark_lava(&mut self, lava: &[Point]) {
        for point in lava.iter() {
            self.set(point, State::Lava);
        }
    }

    fn propagate_free_cubes(&mut self) -> bool {
        let mut has_changed = false;

        for x in 1..self.width {
            for y in 1..self.height {
                for z in 1..self.depth {
                    let point = Point { x, y, z };

                    if matches!(self.get(&point), State::Enclosed)
                        && point
                            .get_neighbors()
                            .any(|neighbor| matches!(self.get(&neighbor), State::Free))
                    {
                        self.set(&point, State::Free);
                        has_changed = true;
                    }
                }
            }
        }

        has_changed
    }

    fn get(&self, point: &Point) -> State {
        self.grid[point.x * self.height * self.depth + point.y * self.depth + point.z]
    }

    fn set(&mut self, point: &Point, state: State) {
        self.grid[point.x * self.height * self.depth + point.y * self.depth + point.z] = state;
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for x in 0..self.width {
            for y in 0..self.height {
                for z in 0..self.depth {
                    match self.get(&Point { x, y, z }) {
                        State::Free => {
                            let _ = write!(f, ".")?;
                        }
                        State::Lava => {
                            let _ = write!(f, "#")?;
                        }
                        State::Enclosed => {
                            let _ = write!(f, "+")?;
                        }
                    }
                }

                let _ = writeln!(f)?;
            }

            let _ = writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    use super::Point;

    fn get_input() -> Vec<Point> {
        vec![
            Point { x: 2, y: 2, z: 2 },
            Point { x: 1, y: 2, z: 2 },
            Point { x: 3, y: 2, z: 2 },
            Point { x: 2, y: 1, z: 2 },
            Point { x: 2, y: 3, z: 2 },
            Point { x: 2, y: 2, z: 1 },
            Point { x: 2, y: 2, z: 3 },
            Point { x: 2, y: 2, z: 4 },
            Point { x: 2, y: 2, z: 6 },
            Point { x: 1, y: 2, z: 5 },
            Point { x: 3, y: 2, z: 5 },
            Point { x: 2, y: 1, z: 5 },
            Point { x: 2, y: 3, z: 5 },
        ]
    }

    #[test]
    fn parsing() {
        let input = r"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

        assert_eq!(
            super::Solver::parse(String::from(input)).unwrap(),
            get_input()
        );
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(super::Solver::part_1(input).unwrap(), 64);
    }

    #[test]
    fn part_2() {
        let input = get_input();

        assert_eq!(super::Solver::part_2(input).unwrap(), 58);
    }
}
