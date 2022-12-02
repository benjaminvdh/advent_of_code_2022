use crate::solving::*;

pub fn solve(input: crate::day_1::Input) -> Result<u64, SolveError> {
    input.into_iter().max().ok_or(SolveError::EmptyInput)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_most_calories() {
        let input = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(solve(input).unwrap(), 24000);
    }
}
