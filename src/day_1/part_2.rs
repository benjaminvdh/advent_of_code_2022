use crate::solving::*;

pub fn solve(input: crate::day_1::Input) -> Result<u64, SolveError> {
    let mut calories: Vec<u32> = input.iter()
        .map(|calories| calories.iter().sum::<u32>())
        .collect();

    calories.sort_unstable();

    let sum: u32 = calories.iter()
        .rev()
        .take(3)
        .sum();

    Ok(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_most_calories() {
        let input = vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000], vec![7000, 8000, 9000], vec![10000]];
        assert_eq!(solve(input).unwrap(), 45000);
    }
}
