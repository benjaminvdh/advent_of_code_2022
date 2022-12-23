use std::convert::{TryFrom, TryInto};

use crate::{ParseError, SolveError};

use crate::day_17::point::Point;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Movement {
    Left,
    Right,
    Forward(usize),
}

impl TryFrom<&str> for Movement {
    type Error = ParseError;

    fn try_from(input: &str) -> Result<Self, ParseError> {
        match input.trim() {
            "R" => Ok(Movement::Right),
            "L" => Ok(Movement::Left),
            number => Ok(Movement::Forward(number.parse().unwrap())),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn right(&self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn left(&self) -> Direction {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }

    fn forward(&self) -> Point {
        match self {
            Direction::Right => Point { x: 1, y: 0 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Up => Point { x: 0, y: -1 },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Open,
    Wall,
    Void,
}

impl TryFrom<char> for Tile {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, ParseError> {
        match c {
            ' ' => Ok(Tile::Void),
            '.' => Ok(Tile::Open),
            '#' => Ok(Tile::Wall),
            _ => Err(ParseError::Invalid),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Map {
    grid: Vec<Tile>,
    width: usize,
    height: usize,
}

#[allow(unused)]
fn print(map: &Map, person: &Person) {
    for y in 0..map.height {
        for x in 0..map.width {
            let p = Point {
                x: x as i64,
                y: y as i64,
            };
            let c = if person.position == p {
                match person.direction {
                    Direction::Right => ">",
                    Direction::Down => "v",
                    Direction::Left => "<",
                    Direction::Up => "^",
                }
            } else {
                match map.get(&p) {
                    Tile::Open => ".",
                    Tile::Wall => "#",
                    Tile::Void => " ",
                }
            };

            print!("{c}");
        }

        println!();
    }

    println!();
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: [Tile::Void].repeat(width * height),
            width,
            height,
        }
    }

    pub fn get_starting_point(&self) -> Result<Point, SolveError> {
        for x in 0..self.width {
            if matches!(self.grid[x], Tile::Open) {
                return Ok(Point { x: x as i64, y: 0 });
            }
        }

        Err(SolveError::InvalidInput)
    }

    pub fn get(&self, pos: &Point) -> Tile {
        if 0 <= pos.x && pos.x < self.width as i64 && 0 <= pos.y && pos.y < self.height as i64 {
            let index = self.to_index(pos);
            self.grid[index]
        } else {
            Tile::Void
        }
    }

    pub fn set(&mut self, pos: &Point, tile: Tile) {
        let index = self.to_index(pos);

        self.grid[index] = tile;
    }

    fn to_index(&self, pos: &Point) -> usize {
        pos.x as usize + pos.y as usize * self.width
    }

    pub fn warp(&self, current_pos: &Point, direction: &Direction) -> Point {
        match direction {
            Direction::Right => {
                let mut x = current_pos.x;
                let y = current_pos.y;

                loop {
                    if x == 0 || matches!(self.get(&Point { x: x - 1, y }), Tile::Void) {
                        break;
                    }

                    x -= 1;
                }

                Point { x, y }
            }
            Direction::Down => {
                let x = current_pos.x;
                let mut y = current_pos.y;

                loop {
                    if y == 0 || matches!(self.get(&Point { x, y: y - 1 }), Tile::Void) {
                        break;
                    }

                    y -= 1;
                }

                Point { x, y }
            }
            Direction::Left => {
                let mut x = current_pos.x;
                let y = current_pos.y;

                loop {
                    if x + 1 == self.width as i64
                        || matches!(self.get(&Point { x: x + 1, y }), Tile::Void)
                    {
                        break;
                    }

                    x += 1;
                }

                Point { x, y }
            }
            Direction::Up => {
                let x = current_pos.x;
                let mut y = current_pos.y;

                loop {
                    if y + 1 == self.height as i64
                        || matches!(self.get(&Point { x, y: y + 1 }), Tile::Void)
                    {
                        break;
                    }

                    y += 1;
                }

                Point { x, y }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Person {
    position: Point,
    direction: Direction,
}

impl Person {
    fn update(&mut self, map: &Map, movement: &Movement) {
        match movement {
            Movement::Right => self.direction = self.direction.right(),
            Movement::Left => self.direction = self.direction.left(),
            Movement::Forward(distance) => self.move_forward(map, *distance),
        }
    }

    fn move_forward(&mut self, map: &Map, distance: usize) {
        let delta = self.direction.forward();

        for _ in 0..distance {
            let new_pos = self.position + delta;

            match map.get(&new_pos) {
                Tile::Open => self.position = new_pos,
                Tile::Wall => return,
                Tile::Void => {
                    let warped_position = map.warp(&self.position, &self.direction);

                    if matches!(map.get(&warped_position), Tile::Wall) {
                        return;
                    } else {
                        self.position = warped_position;
                    }
                }
            }
        }
    }

    fn get_password(&self) -> i64 {
        (self.position.y + 1) * 1000 + (self.position.x + 1) * 4 + self.direction as i64
    }
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = (Map, Vec<Movement>);
    type Output = i64;
    const DAY: u8 = 22;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        let (map, movements) = input.split_once("\n\n").ok_or(ParseError::Invalid)?;

        Ok((parse_map(map)?, parse_movements(movements)?))
    }

    fn part_1(input: Self::Input) -> Result<Self::Output, SolveError> {
        let (map, movements) = input;

        let initial_position = map.get_starting_point()?;

        let mut person = Person {
            position: initial_position,
            direction: Direction::Right,
        };

        for movement in movements {
            person.update(&map, &movement);
        }

        Ok(person.get_password())
    }
}

fn parse_map(input: &str) -> Result<Map, ParseError> {
    let rows: Vec<_> = input
        .lines()
        .map(|line| parse_row(line))
        .collect::<Result<_, _>>()?;

    let width = rows.iter().map(|row| row.len()).max().unwrap_or(0);
    let height = rows.len();

    let mut map = Map::new(width, height);

    for (y, row) in rows.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            map.set(
                &Point {
                    x: x as i64,
                    y: y as i64,
                },
                *tile,
            );
        }
    }

    Ok(map)
}

fn parse_row(input: &str) -> Result<Vec<Tile>, ParseError> {
    Ok(input
        .chars()
        .map(|c| c.try_into())
        .collect::<Result<_, _>>()?)
}

fn parse_movements(input: &str) -> Result<Vec<Movement>, ParseError> {
    let mut movements = vec![];
    let mut prev_index = 0;

    for (index, turn) in input.match_indices(char::is_alphabetic) {
        movements.push(Movement::try_from(&input[prev_index..index])?);
        prev_index = index + 1;

        movements.push(Movement::try_from(turn)?);
    }

    movements.push(Movement::try_from(&input[prev_index..])?);

    Ok(movements)
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    use super::*;

    fn get_input() -> (Map, Vec<Movement>) {
        let width = 16;
        let height = 12;

        let mut map = Map::new(width, height);

        for x in 0..width {
            for y in 0..height {
                map.set(
                    &Point {
                        x: x as i64,
                        y: y as i64,
                    },
                    Tile::Open,
                );
            }
        }

        for x in 0..8 {
            for y in 0..4 {
                map.set(
                    &Point {
                        x: x as i64,
                        y: y as i64,
                    },
                    Tile::Void,
                );
            }
        }

        for x in 0..8 {
            for y in 8..height {
                map.set(
                    &Point {
                        x: x as i64,
                        y: y as i64,
                    },
                    Tile::Void,
                );
            }
        }

        for x in 12..width {
            for y in 0..8 {
                map.set(
                    &Point {
                        x: x as i64,
                        y: y as i64,
                    },
                    Tile::Void,
                );
            }
        }

        for (x, y) in [
            (11, 0),
            (9, 1),
            (8, 2),
            (3, 4),
            (11, 4),
            (8, 5),
            (2, 6),
            (7, 6),
            (10, 7),
            (11, 8),
            (13, 9),
            (9, 10),
            (14, 11),
        ] {
            map.set(&Point { x, y }, Tile::Wall);
        }

        let movements = vec![
            Movement::Forward(10),
            Movement::Right,
            Movement::Forward(5),
            Movement::Left,
            Movement::Forward(5),
            Movement::Right,
            Movement::Forward(10),
            Movement::Left,
            Movement::Forward(4),
            Movement::Right,
            Movement::Forward(5),
            Movement::Left,
            Movement::Forward(5),
        ];

        (map, movements)
    }

    #[test]
    fn parsing() {
        let input = r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

        assert_eq!(
            super::Solver::parse(String::from(input)).unwrap(),
            get_input()
        );
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(super::Solver::part_1(input).unwrap(), 6032);
    }
}
