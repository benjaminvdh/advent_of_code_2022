use crate::ParseError;

use super::{Crates, Task};

pub fn parse(input: String) -> Result<(Crates, Vec<Task>), ParseError> {
    let crates = parse_crates(&input);
    let tasks = parse_tasks(&input)?;

    Ok((crates, tasks))
}

fn parse_crates(input: &str) -> Crates {
    let crate_lines: Vec<_> = input
        .lines()
        .take_while(|line| line.trim_start().starts_with('['))
        .collect();

    crate_lines
        .iter()
        .rev()
        .fold(vec![], |acc, line| add_crates(acc, line))
}

fn add_crates(mut crates: Crates, line: &str) -> Crates {
    let chars: Vec<_> = line.chars().collect();

    for (index, chunk) in chars.chunks(4).enumerate() {
        if let &['[', c, ']', ..] = chunk {
            add_crate(&mut crates, index, c);
        }
    }

    crates
}

fn add_crate(crates: &mut Crates, index: usize, character: char) {
    for _ in crates.len()..=index {
        crates.push(vec![]);
    }

    crates[index].push(character);
}

fn parse_tasks(input: &str) -> Result<Vec<Task>, ParseError> {
    Ok(input
        .lines()
        .skip_while(|line| !line.trim_start().starts_with("move"))
        .map(|line| parse_task(line))
        .collect::<Result<_, _>>()?)
}

fn parse_task(line: &str) -> Result<Task, ParseError> {
    let mut splits = line.split_whitespace();

    let num = parse_as_usize(splits.nth(1))?;
    let from = parse_as_usize(splits.nth(1))?;
    let to = parse_as_usize(splits.nth(1))?;

    Ok(Task { num, from, to })
}

fn parse_as_usize(value: Option<&str>) -> Result<usize, ParseError> {
    value
        .ok_or(ParseError::Invalid)
        .and_then(|value| value.parse().map_err(|e| ParseError::from(e)))
}

#[cfg(test)]
mod tests {
    use super::super::tests::get_input;
    use crate::Solver;

    #[test]
    fn parsing() {
        let input = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let (crates, tasks) = super::super::Solver::parse(String::from(input)).unwrap();
        let (ref_crates, ref_tasks) = get_input();

        assert_eq!(crates, ref_crates);
        assert_eq!(tasks, ref_tasks);
    }
}
