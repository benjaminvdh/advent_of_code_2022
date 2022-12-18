use crate::{ParseError, SolveError};

#[derive(Debug, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    pub fn manhattan_distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
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
        Ok(input.len() * 6
            - input
                .iter()
                .map(|point| {
                    input
                        .iter()
                        .filter(|other| point.manhattan_distance(other) == 1)
                        .count()
                })
                .sum::<usize>())
    }
}

fn parse_line(line: &str) -> Result<Point, ParseError> {
    let mut splits = line.splitn(3, ',');

    let x = splits.next().ok_or(ParseError::Invalid)?.parse()?;
    let y = splits.next().ok_or(ParseError::Invalid)?.parse()?;
    let z = splits.next().ok_or(ParseError::Invalid)?.parse()?;

    Ok(Point { x, y, z })
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
}
