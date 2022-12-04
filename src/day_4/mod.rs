use crate::{ParseError, SolveResult};

pub struct Solver {}

type Range = std::ops::RangeInclusive<u64>;

impl crate::Solver for Solver {
    type Input = Vec<(Range, Range)>;
    const DAY: u8 = 4;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        let pairs = input
            .lines()
            .map(|line| parse_line(line))
            .collect::<Result<_, _>>()?;

        Ok(pairs)
    }

    fn part_1(input: Self::Input) -> SolveResult {
        Ok(input.iter().filter(|(a, b)| ranges_overlap(a, b)).count() as u64)
    }
}

fn parse_line(line: &str) -> Result<(Range, Range), ParseError> {
    let mut splits = line.split(',').map(|range| parse_range(range));

    match (splits.next(), splits.next()) {
        (Some(first), Some(second)) => Ok((first?, second?)),
        _ => Err(ParseError::Incomplete),
    }
}

fn parse_range(range: &str) -> Result<Range, ParseError> {
    let mut splits = range.split('-');

    match (splits.next(), splits.next()) {
        (Some(start), Some(end)) => Ok(start.parse()?..=end.parse()?),
        _ => Err(ParseError::Invalid),
    }
}

fn ranges_overlap(a: &Range, b: &Range) -> bool {
    range_overlaps(a, b) || range_overlaps(b, a)
}

fn range_overlaps(a: &Range, b: &Range) -> bool {
    a.contains(b.start()) && a.contains(b.end())
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    fn get_input() -> <super::Solver as crate::Solver>::Input {
        vec![
            (2..=4, 6..=8),
            (2..=3, 4..=5),
            (5..=7, 7..=9),
            (2..=8, 3..=7),
            (6..=6, 4..=6),
            (2..=6, 4..=8),
        ]
    }

    #[test]
    fn parsing() {
        let input = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let input = super::Solver::parse(String::from(input)).unwrap();
        assert_eq!(input, get_input());
    }

    #[test]
    fn part_1() {
        let input = get_input();
        assert_eq!(super::Solver::part_1(input).unwrap(), 2);
    }
}
