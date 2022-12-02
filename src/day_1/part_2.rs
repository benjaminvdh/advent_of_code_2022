use crate::solving::*;

pub fn solve(input: crate::day_1::Input) -> Result<u64, SolveError> {
    let mut calories: Vec<u64> = input.iter().map(|calories| calories.iter().sum()).collect();

    calories.sort_unstable();

    Ok(calories.iter().rev().take(3).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_most_calories() {
        let input = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        assert_eq!(solve(input).unwrap(), 45000);
    }
}
