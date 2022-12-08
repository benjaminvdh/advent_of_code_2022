use crate::{ParseError, SolveError};

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Vec<Vec<u8>>;
    type Output = usize;
    const DAY: u8 = 8;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        input.lines().map(|line| parse_line(line)).collect()
    }

    fn part_1(input: Self::Input) -> Result<Self::Output, SolveError> {
        Ok((0..input[0].len())
            .map(|col| {
                (0..input.len())
                    .filter(|row| is_visible(&input, col, *row))
                    .count()
            })
            .sum())
    }

    fn part_2(input: Self::Input) -> Result<Self::Output, SolveError> {
        let highest_score = (0..input.len())
            .map(|row| {
                (0..input[row].len())
                    .map(|col| get_scenic_score(&input, col, row))
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0);

        Ok(highest_score)
    }
}

fn parse_line(line: &str) -> Result<Vec<u8>, ParseError> {
    line.chars()
        .map(|c| c.to_digit(10).map(|d| d as u8).ok_or(ParseError::Invalid))
        .collect::<Result<Vec<_>, _>>()
}

fn is_visible(trees: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    let height = trees[j][i];

    let mut left_indices = 0..i;
    let mut right_indices = i + 1..trees.len();
    let mut higher_indices = 0..j;
    let mut lower_indices = j + 1..trees[i].len();

    left_indices.all(|i| trees[j][i] < height)
        || right_indices.all(|i| trees[j][i] < height)
        || higher_indices.all(|j| trees[j][i] < height)
        || lower_indices.all(|j| trees[j][i] < height)
}

fn get_scenic_score(trees: &Vec<Vec<u8>>, col: usize, row: usize) -> usize {
    let height = trees[row][col];

    let mut left = 0;
    for col in (0..col).rev() {
        left += 1;
        if trees[row][col] >= height {
            break;
        }
    }

    let mut right = 0;
    for col in (col + 1)..trees[0].len() {
        right += 1;
        if trees[row][col] >= height {
            break;
        }
    }

    let mut above = 0;
    for row in (0..row).rev() {
        above += 1;
        if trees[row][col] >= height {
            break;
        }
    }

    let mut below = 0;
    for row in (row + 1)..trees.len() {
        below += 1;
        if trees[row][col] >= height {
            break;
        }
    }

    left * right * above * below
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

    #[test]
    fn part_2() {
        assert_eq!(super::Solver::part_2(get_input()).unwrap(), 8)
    }
}
