use crate::solving::*;

pub struct Rucksack(Vec<char>, Vec<char>);

pub type Input = Vec<Rucksack>;

impl Rucksack {
    pub fn new(contents: &str) -> Self {
        let mut items = to_char_vec(contents);
        let mut compartment_2 = items.split_off(items.len() / 2);

        items.sort_unstable();
        compartment_2.sort_unstable();

        Self(items, compartment_2)
    }

    pub fn find_duplicate(&self) -> Result<char, SolveError> {
        self.0
            .iter()
            .find_map(|c| self.1.binary_search(c).ok())
            .map(|i| self.1[i])
            .ok_or(SolveError::InvalidInput)
    }
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

    fn parse<R: Read>(mut input: BufReader<R>) -> Result<Self::Input, ParseError> {
        let mut buf = String::new();
        let _ = input.read_to_string(&mut buf)?;

        Ok(buf.lines().map(|line| Rucksack::new(line)).collect())
    }

    fn part_1(input: Self::Input) -> SolveResult {
        input
            .iter()
            .map(|rucksack| rucksack.find_duplicate().and_then(|c| to_priority(c)))
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

        let mut ref_1 = to_char_vec("vJrwpWtwJgWr");
        ref_1.sort_unstable();
        assert_eq!(rucksack.0, ref_1);

        let mut ref_2 = to_char_vec("hcsFMMfFFhFp");
        ref_2.sort_unstable();
        assert_eq!(rucksack.1, ref_2);
    }

    #[test]
    fn find_duplicate() {
        let rucksack = create_rucksack();
        assert_eq!(rucksack.find_duplicate().unwrap(), 'p');
    }

    #[test]
    fn test_part_1() {
        let rucksacks = vec![
            Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp"),
            Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            Rucksack::new("PmmdzqPrVvPwwTWBwg"),
            Rucksack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            Rucksack::new("ttgJtRGJQctTZtZT"),
            Rucksack::new("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];

        assert_eq!(super::Solver::part_1(rucksacks).unwrap(), 157);
    }
}
