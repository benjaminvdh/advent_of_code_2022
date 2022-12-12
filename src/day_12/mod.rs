use crate::{ParseError, SolveError};

#[derive(Debug)]
struct Node {
    height: u32,
    path_length: Option<usize>,
}

impl From<char> for Node {
    fn from(c: char) -> Self {
        let path_length = if c == 'E' { Some(0) } else { None };

        Self {
            height: c as u32,
            path_length,
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<Node>>,
}

impl Grid {
    fn get_grid_width(&self) -> usize {
        self.grid[0].len()
    }

    fn get_grid_height(&self) -> usize {
        self.grid.len()
    }

    fn get_height(&self, x: usize, y: usize) -> u32 {
        let height = self.grid[y][x].height;

        if height == 'E' as u32 {
            'z' as u32
        } else if height == 'S' as u32 {
            'a' as u32
        } else {
            height
        }
    }

    fn get_path_length(&self, x: usize, y: usize) -> Option<usize> {
        self.grid[y][x].path_length
    }

    fn find_start(&self) -> Option<(usize, usize)> {
        self.find_node_with_height('S' as u32)
    }

    fn find_end(&self) -> Option<(usize, usize)> {
        self.find_node_with_height('E' as u32)
    }

    fn find_node_with_height(&self, height: u32) -> Option<(usize, usize)> {
        for i in 0..self.get_grid_width() {
            for j in 0..self.get_grid_height() {
                if self.grid[j][i].height == height {
                    return Some((i, j));
                }
            }
        }

        None
    }

    fn visit_neighbors(&mut self, x: usize, y: usize) {
        if x > 0 {
            self.visit_neighbor(x - 1, y, x, y);
        }

        if x + 1 < self.get_grid_width() {
            self.visit_neighbor(x + 1, y, x, y);
        }

        if y > 0 {
            self.visit_neighbor(x, y - 1, x, y);
        }

        if y + 1 < self.get_grid_height() {
            self.visit_neighbor(x, y + 1, x, y);
        }
    }

    fn visit_neighbor(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) {
        let current_path_length = self.get_path_length(to_x, to_y).unwrap();

        if self.is_shorter_path(from_x, from_y, to_x, to_y) {
            self.grid[from_y][from_x].path_length = Some(current_path_length + 1);
            self.visit_neighbors(from_x, from_y);
        }
    }

    fn is_shorter_path(&self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool {
        self.get_height(to_x, to_y) <= self.get_height(from_x, from_y) + 1
            && self
                .get_path_length(from_x, from_y)
                .map(|length| length > self.get_path_length(to_x, to_y).unwrap() + 1)
                .unwrap_or(true)
    }

    #[allow(unused)]
    fn print_heights(&self) {
        for j in 0..self.get_grid_height() {
            for i in 0..self.get_grid_width() {
                print!("{}", char::from_u32(self.grid[j][i].height).unwrap());
            }
            println!()
        }
    }

    #[allow(unused)]
    fn print_path_lengths(&self) {
        for j in 0..self.get_grid_height() {
            for i in 0..self.get_grid_width() {
                print!(
                    " {:>2} ",
                    self.grid[j][i]
                        .path_length
                        .map(|l| l.to_string())
                        .unwrap_or(String::from("."))
                );
            }
            println!()
        }
    }
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Grid;
    type Output = usize;
    const DAY: u8 = 12;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        Ok(Grid {
            grid: input
                .lines()
                .map(|line| line.chars().map(|c| Node::from(c)).collect())
                .collect(),
        })
    }

    fn part_1(mut grid: Self::Input) -> Result<Self::Output, SolveError> {
        let (x, y) = grid.find_end().ok_or(SolveError::InvalidInput)?;

        grid.visit_neighbors(x, y);

        let (x, y) = grid.find_start().ok_or(SolveError::InvalidInput)?;
        grid.get_path_length(x, y).ok_or(SolveError::InvalidInput)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    use super::*;

    fn get_input() -> Grid {
        Grid {
            grid: vec![
                vec![
                    Node::from('S'),
                    Node::from('a'),
                    Node::from('b'),
                    Node::from('q'),
                    Node::from('p'),
                    Node::from('o'),
                    Node::from('n'),
                    Node::from('m'),
                ],
                vec![
                    Node::from('a'),
                    Node::from('b'),
                    Node::from('c'),
                    Node::from('r'),
                    Node::from('y'),
                    Node::from('x'),
                    Node::from('x'),
                    Node::from('l'),
                ],
                vec![
                    Node::from('a'),
                    Node::from('c'),
                    Node::from('c'),
                    Node::from('s'),
                    Node::from('z'),
                    Node::from('E'),
                    Node::from('x'),
                    Node::from('k'),
                ],
                vec![
                    Node::from('a'),
                    Node::from('c'),
                    Node::from('c'),
                    Node::from('t'),
                    Node::from('u'),
                    Node::from('v'),
                    Node::from('w'),
                    Node::from('j'),
                ],
                vec![
                    Node::from('a'),
                    Node::from('b'),
                    Node::from('d'),
                    Node::from('e'),
                    Node::from('f'),
                    Node::from('g'),
                    Node::from('h'),
                    Node::from('i'),
                ],
            ],
        }
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(super::Solver::part_1(input).unwrap(), 31);
    }
}
