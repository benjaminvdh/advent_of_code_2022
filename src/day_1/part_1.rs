use crate::solving::*;

pub fn solve(input: &crate::day_1::Input) -> Result<u64, SolveError> {
    input.iter()
        .map(|calories| calories.iter().sum::<u32>())
        .max()
        .map(|calories| calories as u64)
        .ok_or(SolveError::EmptyInput)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_most_calories() {
        let input = vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000], vec![7000, 8000, 9000], vec![10000]];
        assert_eq!(solve(&input).unwrap(), 24000);
    }
}
