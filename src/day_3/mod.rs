use std::collections::HashSet;
use std::iter::FromIterator;

use crate::{ParseError, SolveError, SolveResult};

pub struct Rucksack(HashSet<char>, HashSet<char>);

pub type Input = Vec<Rucksack>;

impl Rucksack {
    pub fn new(contents: &str) -> Self {
        let mut items = to_char_vec(contents);
        let compartment_2 = items.split_off(items.len() / 2);

        Self(
            HashSet::from_iter(items.into_iter()),
            HashSet::from_iter(compartment_2.into_iter()),
        )
    }

    pub fn find_duplicate(&self) -> Result<char, SolveError> {
        let duplicates = find_duplicates(&self.0, &self.1);

        if duplicates.len() == 1 {
            Ok(*duplicates.iter().next().unwrap())
        } else {
            Err(SolveError::InvalidInput)
        }
    }

    pub fn find_badge(&self, second: &Rucksack, third: &Rucksack) -> Result<char, SolveError> {
        let shared_with_second = find_duplicates(&self.get_all_items(), &second.get_all_items());
        let shared_between_all = find_duplicates(&shared_with_second, &third.get_all_items());

        if shared_between_all.len() == 1 {
            Ok(*shared_between_all.iter().next().unwrap())
        } else {
            Err(SolveError::InvalidInput)
        }
    }

    fn get_all_items(&self) -> HashSet<char> {
        HashSet::from_iter(self.0.iter().chain(self.1.iter()).map(|c| *c))
    }
}

fn find_duplicates<'a>(a: &HashSet<char>, b: &HashSet<char>) -> HashSet<char> {
    a.intersection(b).map(|c| *c).collect()
}

fn to_char_vec(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn to_priority(c: char) -> Result<u64, SolveError> {
    if c.is_ascii_lowercase() {
        Ok((c as u64) - ('a' as u64) + 1)
    } else if c.is_ascii_uppercase() {
        Ok((c as u64) - ('A' as u64) + 27)
    } else {
        Err(SolveError::InvalidInput)
    }
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Input;
    const DAY: u8 = 3;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        Ok(input.lines().map(|line| Rucksack::new(line)).collect())
    }

    fn part_1(input: Self::Input) -> SolveResult {
        input
            .iter()
            .map(|rucksack| rucksack.find_duplicate().and_then(|c| to_priority(c)))
            .sum()
    }

    fn part_2(input: Self::Input) -> SolveResult {
        input
            .chunks_exact(3)
            .map(|chunk| {
                chunk[0]
                    .find_badge(&chunk[1], &chunk[2])
                    .and_then(|c| to_priority(c))
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::solving::Solver;

    use super::*;

    fn create_rucksack() -> Rucksack {
        Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp")
    }

    #[test]
    fn test_priority() {
        assert_eq!(to_priority('p').unwrap(), 16);
        assert_eq!(to_priority('L').unwrap(), 38);
        assert_eq!(to_priority('P').unwrap(), 42);
        assert_eq!(to_priority('v').unwrap(), 22);
        assert_eq!(to_priority('t').unwrap(), 20);
        assert_eq!(to_priority('s').unwrap(), 19);
    }

    #[test]
    fn new_rucksack() {
        let rucksack = create_rucksack();

        let ref_1 = HashSet::from_iter("vJrwpWtwJgWr".chars());
        assert_eq!(rucksack.0, ref_1);

        let ref_2 = HashSet::from_iter("hcsFMMfFFhFp".chars());
        assert_eq!(rucksack.1, ref_2);
    }

    #[test]
    fn find_duplicate() {
        let rucksack = create_rucksack();
        assert_eq!(rucksack.find_duplicate().unwrap(), 'p');
    }

    fn get_rucksacks() -> Vec<Rucksack> {
        vec![
            Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp"),
            Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            Rucksack::new("PmmdzqPrVvPwwTWBwg"),
            Rucksack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            Rucksack::new("ttgJtRGJQctTZtZT"),
            Rucksack::new("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ]
    }

    #[test]
    fn part_1() {
        let rucksacks = get_rucksacks();
        assert_eq!(super::Solver::part_1(rucksacks).unwrap(), 157);
    }

    #[test]
    fn part_2() {
        let rucksacks = get_rucksacks();
        assert_eq!(super::Solver::part_2(rucksacks).unwrap(), 70);
    }
}
