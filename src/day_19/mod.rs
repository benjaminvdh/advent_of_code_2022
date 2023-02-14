use std::cmp::Ordering;
use std::ops::SubAssign;
use std::thread;

use crate::{ParseError, SolveError};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Resources) -> Option<Ordering> {
        if self.ore < other.ore
            || self.clay < other.clay
            || self.obsidian < other.obsidian
            || self.geodes < other.geodes
        {
            Some(Ordering::Less)
        } else if self.ore > other.ore
            || self.clay > other.clay
            || self.obsidian > other.obsidian
            || self.geodes > other.geodes
        {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl SubAssign<&Resources> for Resources {
    fn sub_assign(&mut self, rhs: &Resources) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
        self.geodes -= rhs.geodes;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Blueprint {
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

#[derive(Clone, Copy, Debug)]
struct Factory<'a> {
    resources: Resources,
    blueprint: &'a Blueprint,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
}

impl<'a> Factory<'a> {
    fn new(blueprint: &'a Blueprint) -> Self {
        Self {
            resources: Resources::default(),
            blueprint,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }

    fn run(&self, time: usize) -> usize {
        if time == 24 {
            self.resources.geodes
        } else if self.can_build_geode_robot() {
            self.build_geode_robot().run(time + 1)
        } else {
            let mut max = 0;

            if self.can_build_obsidian_robot() {
                max = self.build_obsidian_robot().run(time + 1);
            }

            if self.can_build_ore_robot() {
                max = max.max(self.build_ore_robot().run(time + 1));
            }

            if self.can_build_clay_robot() {
                max = max.max(self.build_clay_robot().run(time + 1));
            }

            max = max.max(self.build_nothing().run(time + 1));

            max
        }
    }

    fn update(&mut self) {
        self.resources.ore += self.ore_robots;
        self.resources.clay += self.clay_robots;
        self.resources.obsidian += self.obsidian_robots;
        self.resources.geodes += self.geode_robots;
    }

    fn can_build_ore_robot(&self) -> bool {
        self.resources >= self.blueprint.ore_robot_cost
            && self.ore_robots
                < self
                    .blueprint
                    .ore_robot_cost
                    .ore
                    .max(self.blueprint.clay_robot_cost.ore)
                    .max(self.blueprint.obsidian_robot_cost.ore)
                    .max(self.blueprint.geode_robot_cost.ore)
    }

    fn can_build_clay_robot(&self) -> bool {
        self.resources >= self.blueprint.clay_robot_cost
            && self.clay_robots < self.blueprint.obsidian_robot_cost.clay
    }

    fn can_build_obsidian_robot(&self) -> bool {
        self.resources >= self.blueprint.obsidian_robot_cost
            && self.obsidian_robots < self.blueprint.geode_robot_cost.obsidian
    }

    fn can_build_geode_robot(&self) -> bool {
        self.resources >= self.blueprint.geode_robot_cost
    }

    fn build_ore_robot(mut self) -> Self {
        self.resources -= &self.blueprint.ore_robot_cost;
        self.update();
        self.ore_robots += 1;
        self
    }

    fn build_clay_robot(mut self) -> Self {
        self.resources -= &self.blueprint.clay_robot_cost;
        self.update();
        self.clay_robots += 1;
        self
    }

    fn build_obsidian_robot(mut self) -> Self {
        self.resources -= &self.blueprint.obsidian_robot_cost;
        self.update();
        self.obsidian_robots += 1;
        self
    }

    fn build_geode_robot(mut self) -> Self {
        self.resources -= &self.blueprint.geode_robot_cost;
        self.update();
        self.geode_robots += 1;
        self
    }

    fn build_nothing(mut self) -> Self {
        self.update();
        self
    }
}

fn parse_line(line: &str) -> Result<Blueprint, ParseError> {
    let numbers: Vec<usize> = line
        .split_whitespace()
        .filter_map(|substring| substring.parse::<usize>().ok())
        .collect();

    if numbers.len() == 6 {
        Ok(Blueprint {
            ore_robot_cost: Resources {
                ore: numbers[0],
                ..Default::default()
            },
            clay_robot_cost: Resources {
                ore: numbers[1],
                ..Default::default()
            },
            obsidian_robot_cost: Resources {
                ore: numbers[2],
                clay: numbers[3],
                ..Default::default()
            },
            geode_robot_cost: Resources {
                ore: numbers[4],
                obsidian: numbers[5],
                ..Default::default()
            },
        })
    } else {
        Err(ParseError::Invalid)
    }
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Vec<Blueprint>;
    type Output = usize;
    const DAY: u8 = 19;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        input
            .lines()
            .map(|line| parse_line(line))
            .collect::<Result<Vec<_>, _>>()
    }

    fn part_1(blueprints: Self::Input) -> Result<Self::Output, SolveError> {
        let mut handles = vec![];

        for (i, blueprint) in blueprints.into_iter().enumerate() {
            handles.push(thread::spawn(move || {
                let i = i + 1;
                let result = Factory::new(&blueprint).run(0);

                eprintln!("The quality level of blueprint {i} is {result}");

                i * result
            }));
        }

        let mut sum = 0;

        for handle in handles {
            sum += handle.join().map_err(|_| SolveError::InvalidInput)?;
        }

        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    use super::{Blueprint, Resources};

    fn get_input() -> Vec<Blueprint> {
        vec![
            Blueprint {
                ore_robot_cost: Resources {
                    ore: 4,
                    ..Default::default()
                },
                clay_robot_cost: Resources {
                    ore: 2,
                    ..Default::default()
                },
                obsidian_robot_cost: Resources {
                    ore: 3,
                    clay: 14,
                    ..Default::default()
                },
                geode_robot_cost: Resources {
                    ore: 2,
                    obsidian: 7,
                    ..Default::default()
                },
            },
            Blueprint {
                ore_robot_cost: Resources {
                    ore: 2,
                    ..Default::default()
                },
                clay_robot_cost: Resources {
                    ore: 3,
                    ..Default::default()
                },
                obsidian_robot_cost: Resources {
                    ore: 3,
                    clay: 8,
                    ..Default::default()
                },
                geode_robot_cost: Resources {
                    ore: 3,
                    obsidian: 12,
                    ..Default::default()
                },
            },
        ]
    }

    #[test]
    fn parsing() {
        let input = r"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

        assert_eq!(
            super::Solver::parse(String::from(input)).unwrap(),
            get_input()
        );
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(super::Solver::part_1(input).unwrap(), 33);
    }
}
