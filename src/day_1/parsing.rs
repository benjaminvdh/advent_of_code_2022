use crate::parsing::*;

pub fn parse<R: Read>(input: BufReader<R>) -> Result<crate::day_1::Input, ParseError> {
    let mut elves = vec![];
    let mut elf = vec![];

    for line in input.lines() {
        let line = line?;

        if !line.is_empty() {
            elf.push(line.parse()?);
        } else {
            elves.push(elf);
            elf = vec![];
        }
    }

    elves.push(elf);

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
        let ref_input = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        assert_eq!(parsed_input, ref_input);
    }
}
