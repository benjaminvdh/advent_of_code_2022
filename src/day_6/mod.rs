use crate::{ParseError, SolveError};

fn all_different(window: &[char]) -> bool {
    for i in 0..window.len() {
        for j in i + 1..window.len() {
            if window[i] == window[j] {
                return false;
            }
        }
    }

    true
}

pub struct Solver;

impl crate::Solver for Solver {
    type Input = Vec<char>;
    type Output = usize;
    const DAY: u8 = 6;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        Ok(input.chars().collect())
    }

    fn part_1(input: Self::Input) -> Result<Self::Output, SolveError> {
        input
            .windows(4)
            .enumerate()
            .find(|(_, window)| all_different(window))
            .map(|(index, _)| index + 4)
            .ok_or(SolveError::InvalidInput)
    }
}
#[cfg(test)]
mod tests {
    use crate::Solver;

    #[test]
    fn part_1_a() {
        let input = super::Solver::parse(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")).unwrap();
        assert_eq!(super::Solver::part_1(input).unwrap(), 7);
    }

    #[test]
    fn part_1_b() {
        let input = super::Solver::parse(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")).unwrap();
        assert_eq!(super::Solver::part_1(input).unwrap(), 5);
    }

    #[test]
    fn part_1_c() {
        let input = super::Solver::parse(String::from("nppdvjthqldpwncqszvftbrmjlhg")).unwrap();
        assert_eq!(super::Solver::part_1(input).unwrap(), 6);
    }

    #[test]
    fn part_1_d() {
        let input =
            super::Solver::parse(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")).unwrap();
        assert_eq!(super::Solver::part_1(input).unwrap(), 10);
    }

    #[test]
    fn part_1_e() {
        let input = super::Solver::parse(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")).unwrap();
        assert_eq!(super::Solver::part_1(input).unwrap(), 11);
    }
}
