use crate::{ParseError, SolveError};

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Vec<Vec<u8>>;
    type Output = u32;
    const DAY: u8 = 8;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        input.lines().map(|line| parse_line(line)).collect()
    }

    fn part_1(input: Self::Input) -> Result<Self::Output, SolveError> {
        let width = input.len();
        let height = input[0].len();

        let mut visible_trees = 0;
        for i in 0..width {
            for j in 0..height {
                if is_visible(&input, i, j) {
                    visible_trees += 1;
                }
            }
        }

        Ok(visible_trees)
    }
}

fn parse_line(line: &str) -> Result<Vec<u8>, ParseError> {
    line.chars()
        .map(|c| c.to_digit(10).map(|d| d as u8).ok_or(ParseError::Invalid))
        .collect::<Result<Vec<_>, _>>()
}

fn is_visible(trees: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    let height = trees[i][j];

    let mut left_indices = 0..i;
    let mut right_indices = i + 1..trees.len();
    let mut higher_indices = 0..j;
    let mut lower_indices = j + 1..trees[i].len();

    left_indices.all(|i| trees[i][j] < height)
        || right_indices.all(|i| trees[i][j] < height)
        || higher_indices.all(|j| trees[i][j] < height)
        || lower_indices.all(|j| trees[i][j] < height)
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    fn get_input() -> Vec<Vec<u8>> {
        vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]
    }

    #[test]
    fn parsing() {
        let input = r"30373
25512
65332
33549
35390";

        assert_eq!(
            super::Solver::parse(String::from(input)).unwrap(),
            get_input()
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(super::Solver::part_1(get_input()).unwrap(), 21)
    }
}
