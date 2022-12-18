use std::collections::BTreeSet;

use super::{Point, Rock};

#[derive(Default)]
pub struct Chamber {
    points: [BTreeSet<i64>; Self::WIDTH as usize],
}

impl Chamber {
    pub const WIDTH: i64 = 7;

    pub fn spawn_rock(&self, pattern: &'static [Point]) -> Rock {
        Rock {
            pattern,
            position: Point::new(2, self.get_height() + 4),
        }
    }

    pub fn place_rock(&mut self, rock: &Rock) {
        for point in rock.get_points() {
            let _ = self.points[point.x as usize].insert(point.y);
        }
    }

    pub fn get_height(&self) -> i64 {
        self.points
            .iter()
            .map(|yy| *yy.iter().next_back().unwrap_or(&0))
            .max()
            .unwrap_or(0)
    }

    pub fn can_move(&self, rock: &Rock, movement: &Point) -> bool {
        let rock = rock + movement;

        let can_move = !rock.get_points().any(|point| self.has_collision(&point));
        can_move
    }

    #[allow(unused)]
    pub fn is_blocked(&self, point: &Point) -> bool {
        self.points[point.x as usize].contains(&point.y)
    }

    fn has_collision(&self, point: &Point) -> bool {
        point.x < 0
            || Self::WIDTH <= point.x
            || point.y <= 0
            || self.points[point.x as usize].contains(&point.y)
    }
}
