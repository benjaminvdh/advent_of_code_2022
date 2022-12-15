use std::collections::BTreeSet;

use crate::{ParseError, SolveError};

pub type Point = (i32, i32);

#[derive(Debug, PartialEq)]
pub struct Sensor {
    position: Point,
    nearest_beacon: Point,
    nearest_distance: i32,
}

impl Sensor {
    fn new(position: Point, nearest_beacon: Point) -> Self {
        let nearest_distance = get_distance(&position, &nearest_beacon);

        Self {
            position,
            nearest_beacon,
            nearest_distance,
        }
    }
}

fn get_distance(a: &Point, b: &Point) -> i32 {
    let (ax, ay) = a;
    let (bx, by) = b;

    ax.max(bx) - ax.min(bx) + ay.max(by) - ay.min(by)
}

pub struct Solver<const Y: i32> {}

impl<const Y: i32> crate::Solver for Solver<Y> {
    type Input = Vec<Sensor>;
    type Output = usize;
    const DAY: u8 = 15;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        input
            .lines()
            .map(|line| parse_line(line))
            .collect::<Result<_, _>>()
    }

    fn part_1(sensors: Self::Input) -> Result<Self::Output, SolveError> {
        let mut points = BTreeSet::new();

        for sensor in sensors.iter() {
            for x in -sensor.nearest_distance..sensor.nearest_distance {
                let point = (sensor.position.0 + x, Y);

                if get_distance(&sensor.position, &point) <= sensor.nearest_distance {
                    let _ = points.insert(point);
                }
            }
        }

        let beacons: BTreeSet<_> = sensors.iter().map(|sensor| sensor.nearest_beacon).collect();

        Ok(points.difference(&beacons).filter(|p| p.1 == Y).count())
    }
}

fn parse_line(line: &str) -> Result<Sensor, ParseError> {
    let line = line
        .strip_prefix("Sensor at x=")
        .ok_or(ParseError::Invalid)?;
    let (sensor_x, line) = line.split_once(",").ok_or(ParseError::Invalid)?;
    let line = line.strip_prefix(" y=").ok_or(ParseError::Invalid)?;
    let (sensor_y, line) = line.split_once(":").ok_or(ParseError::Invalid)?;
    let line = line
        .strip_prefix(" closest beacon is at x=")
        .ok_or(ParseError::Invalid)?;
    let (beacon_x, line) = line.split_once(",").ok_or(ParseError::Invalid)?;
    let beacon_y = line.strip_prefix(" y=").ok_or(ParseError::Invalid)?;

    Ok(Sensor::new(
        (sensor_x.parse()?, sensor_y.parse()?),
        (beacon_x.parse()?, beacon_y.parse()?),
    ))
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    use super::Sensor;

    fn get_input() -> Vec<Sensor> {
        vec![
            Sensor::new((2, 18), (-2, 15)),
            Sensor::new((9, 16), (10, 16)),
            Sensor::new((13, 2), (15, 3)),
            Sensor::new((12, 14), (10, 16)),
            Sensor::new((10, 20), (10, 16)),
            Sensor::new((14, 17), (10, 16)),
            Sensor::new((8, 7), (2, 10)),
            Sensor::new((2, 0), (2, 10)),
            Sensor::new((0, 11), (2, 10)),
            Sensor::new((20, 14), (25, 17)),
            Sensor::new((17, 20), (21, 22)),
            Sensor::new((16, 7), (15, 3)),
            Sensor::new((14, 3), (15, 3)),
            Sensor::new((20, 1), (15, 3)),
        ]
    }

    #[test]
    fn parsing() {
        let input = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        assert_eq!(
            super::Solver::<10>::parse(String::from(input)).unwrap(),
            get_input()
        );
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(super::Solver::<10>::part_1(input).unwrap(), 26);
    }
}
