use crate::{ParseError, SolveResult};

pub struct Rucksack(u64, u64);

pub type Input = Vec<Rucksack>;

impl Rucksack {
    pub fn new(contents: &str) -> Result<Self, ParseError> {
        if input_is_valid(contents) {
            let (compartment_1, compartment_2) = contents.split_at(contents.len() / 2);
            Ok(Self(to_u64(compartment_1), to_u64(compartment_2)))
        } else {
            Err(ParseError::Invalid)
        }
    }

    pub fn find_duplicate(&self) -> char {
        from_u64(self.0 & self.1)
    }
}

fn input_is_valid(input: &str) -> bool {
    input
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_uppercase())
}

fn find_badge(first: &Rucksack, second: &Rucksack, third: &Rucksack) -> char {
    from_u64((first.0 | first.1) & (second.0 | second.1) & (third.0 | third.1))
}

fn to_u64(input: &str) -> u64 {
    input.chars().fold(0, |acc, c| acc | 1 << to_priority(c))
}

fn from_u64(input: u64) -> char {
    let priority = input.trailing_zeros() as u8;

    match priority {
        1..=26 => (priority - 1 + 'a' as u8) as char,
        27..=52 => (priority - 27 + 'A' as u8) as char,
        _ => unreachable!(),
    }
}

fn to_priority(c: char) -> u64 {
    if c.is_ascii_lowercase() {
        (c as u64) - ('a' as u64) + 1
    } else if c.is_ascii_uppercase() {
        (c as u64) - ('A' as u64) + 27
    } else {
        unreachable!()
    }
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Input;
    const DAY: u8 = 3;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        Ok(input
            .lines()
            .map(|line| Rucksack::new(line))
            .collect::<Result<_, _>>()?)
    }

    fn part_1(input: Self::Input) -> SolveResult {
        Ok(input
            .iter()
            .map(|rucksack| to_priority(rucksack.find_duplicate()))
            .sum())
    }

    fn part_2(input: Self::Input) -> SolveResult {
        Ok(input
            .chunks_exact(3)
            .map(|chunk| to_priority(find_badge(&chunk[0], &chunk[1], &chunk[2])))
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use crate::solving::Solver;

    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(to_priority('p'), 16);
        assert_eq!(to_priority('L'), 38);
        assert_eq!(to_priority('P'), 42);
        assert_eq!(to_priority('v'), 22);
        assert_eq!(to_priority('t'), 20);
        assert_eq!(to_priority('s'), 19);
    }

    #[test]
    fn find_duplicate() {
        let rucksack = Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap();
        assert_eq!(rucksack.find_duplicate(), 'p');
    }

    fn get_rucksacks() -> Vec<Rucksack> {
        vec![
            Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap(),
            Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").unwrap(),
            Rucksack::new("PmmdzqPrVvPwwTWBwg").unwrap(),
            Rucksack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").unwrap(),
            Rucksack::new("ttgJtRGJQctTZtZT").unwrap(),
            Rucksack::new("CrZsJsPPZsGzwwsLwLmpwMDw").unwrap(),
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
