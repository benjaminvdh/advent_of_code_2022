use std::ops::Add;

use super::Point;

#[derive(Copy, Clone)]
pub struct Rock {
    pub pattern: &'static [Point],
    pub position: Point,
}

impl Rock {
    pub fn get_points<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        self.pattern.iter().map(move |point| point + self.position)
    }
}

impl Add<&Point> for &Rock {
    type Output = Rock;

    fn add(self, point: &Point) -> Self::Output {
        Rock {
            pattern: self.pattern,
            position: self.position + point,
        }
    }
}

pub const PATTERNS: [&[Point]; 5] = [
    &[
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(3, 0),
    ],
    &[
        Point::new(0, 1),
        Point::new(1, 1),
        Point::new(2, 1),
        Point::new(1, 0),
        Point::new(1, 2),
    ],
    &[
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(2, 1),
        Point::new(2, 2),
    ],
    &[
        Point::new(0, 0),
        Point::new(0, 1),
        Point::new(0, 2),
        Point::new(0, 3),
    ],
    &[
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(1, 1),
    ],
];
