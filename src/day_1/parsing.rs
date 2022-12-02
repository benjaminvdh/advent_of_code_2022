use crate::parsing::*;

pub fn parse<R: Read>(mut input: BufReader<R>) -> Result<crate::day_1::Input, ParseError> {
    let mut contents = String::new();
    let _ = input.read_to_string(&mut contents)?;

    let elves = contents
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u64>()).sum())
        .collect::<Result<_, _>>()?;

    Ok(elves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input() {
        let input = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let parsed_input = parse(BufReader::new(input.as_bytes())).unwrap();
        let ref_input = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(parsed_input, ref_input);
    }
}
