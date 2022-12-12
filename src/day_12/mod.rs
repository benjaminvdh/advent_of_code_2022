use crate::{ParseError, SolveError};

#[derive(Debug, PartialEq)]
struct Node {
    height: u32,
    path_length: Option<usize>,
}

impl Node {
    fn get_height(&self) -> u32 {
        if self.height == 'E' as u32 {
            'z' as u32
        } else if self.height == 'S' as u32 {
            'a' as u32
        } else {
            self.height
        }
    }
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

#[derive(Debug, PartialEq)]
pub struct Grid {
    grid: Vec<Vec<Node>>,
}

impl Grid {
    fn find_start_node(&self) -> Result<(usize, usize), SolveError> {
        self.find_node_with_height('S' as u32)
            .ok_or(SolveError::InvalidInput)
    }

    fn find_end_node(&self) -> Result<(usize, usize), SolveError> {
        self.find_node_with_height('E' as u32)
            .ok_or(SolveError::InvalidInput)
    }

    fn find_node_with_height(&self, height: u32) -> Option<(usize, usize)> {
        self.grid.iter().enumerate().find_map(|(j, row)| {
            row.iter().enumerate().find_map(|(i, node)| {
                if node.height == height {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
    }

    fn visit_neighbors(&mut self, x: usize, y: usize) {
        if x > 0 {
            self.visit_neighbor(x - 1, y, x, y);
        }

        if x + 1 < self.grid[0].len() {
            self.visit_neighbor(x + 1, y, x, y);
        }

        if y > 0 {
            self.visit_neighbor(x, y - 1, x, y);
        }

        if y + 1 < self.grid.len() {
            self.visit_neighbor(x, y + 1, x, y);
        }
    }

    fn visit_neighbor(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) {
        if self.is_shorter_path(from_x, from_y, to_x, to_y) {
            let length = self.grid[to_y][to_x].path_length.unwrap();
            self.grid[from_y][from_x].path_length = Some(length + 1);

            self.visit_neighbors(from_x, from_y);
        }
    }

    fn is_shorter_path(&self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool {
        self.grid[to_y][to_x].get_height() <= self.grid[from_y][from_x].get_height() + 1
            && self.grid[from_y][from_x]
                .path_length
                .map(|length| length > self.grid[to_y][to_x].path_length.unwrap() + 1)
                .unwrap_or(true)
    }
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Grid;
    type Output = usize;
    const DAY: u8 = 12;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        let grid = input
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();

        Ok(Grid { grid })
    }

    fn part_1(mut grid: Self::Input) -> Result<Self::Output, SolveError> {
        let (x, y) = grid.find_end_node()?;
        grid.visit_neighbors(x, y);

        let (x, y) = grid.find_start_node()?;
        grid.grid[y][x].path_length.ok_or(SolveError::InvalidInput)
    }

    fn part_2(mut grid: Self::Input) -> Result<Self::Output, SolveError> {
        let (x, y) = grid.find_end_node()?;

        grid.visit_neighbors(x, y);

        grid.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|node| node.get_height() == 'a' as u32)
            .filter_map(|node| node.path_length)
            .min()
            .ok_or(SolveError::InvalidInput)
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
    fn parsing() {
        let input = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        assert_eq!(
            super::Solver::parse(String::from(input)).unwrap(),
            get_input()
        );
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(super::Solver::part_1(input).unwrap(), 31);
    }

    #[test]
    fn part_2() {
        let input = get_input();

        assert_eq!(super::Solver::part_2(input).unwrap(), 29);
    }
}
