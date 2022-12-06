use crate::{ParseError, SolveError};

fn get_index(input: Vec<char>, window_size: usize) -> Result<usize, SolveError> {
    input
        .windows(window_size)
        .enumerate()
        .find(|(_, window)| all_different(window))
        .map(|(index, window)| index + window.len())
        .ok_or(SolveError::InvalidInput)
}

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
        get_index(input, 4)
    }

    fn part_2(input: Self::Input) -> Result<Self::Output, SolveError> {
        get_index(input, 14)
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

    #[test]
    fn part_2_a() {
        let input = super::Solver::parse(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")).unwrap();
        assert_eq!(super::Solver::part_2(input).unwrap(), 19);
    }

    #[test]
    fn part_2_b() {
        let input = super::Solver::parse(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")).unwrap();
        assert_eq!(super::Solver::part_2(input).unwrap(), 23);
    }

    #[test]
    fn part_2_c() {
        let input = super::Solver::parse(String::from("nppdvjthqldpwncqszvftbrmjlhg")).unwrap();
        assert_eq!(super::Solver::part_2(input).unwrap(), 23);
    }

    #[test]
    fn part_2_d() {
        let input =
            super::Solver::parse(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")).unwrap();
        assert_eq!(super::Solver::part_2(input).unwrap(), 29);
    }

    #[test]
    fn part_2_e() {
        let input = super::Solver::parse(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")).unwrap();
        assert_eq!(super::Solver::part_2(input).unwrap(), 26);
    }
}
