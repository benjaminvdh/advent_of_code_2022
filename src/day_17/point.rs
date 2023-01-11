use std::borrow::Borrow;
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub const LEFT: Point = Point::new(-1, 0);
    pub const RIGHT: Point = Point::new(1, 0);
    pub const DOWN: Point = Point::new(0, -1);

    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl<P: Borrow<Point>> Add<P> for Point {
    type Output = Point;

    fn add(self, rhs: P) -> Self::Output {
        &self + rhs
    }
}

impl<P: Borrow<Point>> Add<P> for &Point {
    type Output = Point;

    fn add(self, rhs: P) -> Self::Output {
        Self::Output {
            x: self.x + rhs.borrow().x,
            y: self.y + rhs.borrow().y,
        }
    }
}

impl<P: Borrow<Point>> AddAssign<P> for Point {
    fn add_assign(&mut self, rhs: P) {
        self.x += rhs.borrow().x;
        self.y += rhs.borrow().y;
    }
}
