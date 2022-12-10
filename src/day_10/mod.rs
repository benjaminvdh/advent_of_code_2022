use crate::{ParseError, SolveError};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    fn execute(&self, x: &mut i32, clock: &mut i32, signal_strengths: &mut [i32; 6]) {
        for i in 0..6 {
            if *clock == 20 + (i as i32) * 40 {
                signal_strengths[i] = *clock * *x;
            }
        }

        match self {
            Instruction::Addx(value) => {
                for i in 0..6 {
                    if *clock == 19 + (i as i32) * 40 {
                        signal_strengths[i] = (*clock + 1) * *x;
                    }
                }
                *x += value;
                *clock += 2;
            }
            Instruction::Noop => {
                *clock += 1;
            }
        }
    }
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Vec<Instruction>;
    type Output = i32;
    const DAY: u8 = 10;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        input.lines().map(|line| parse_line(line)).collect()
    }

    fn part_1(input: Self::Input) -> Result<Self::Output, SolveError> {
        let mut x = 1;
        let mut clock = 1;
        let mut signal_strengths = [0; 6];

        for instruction in input {
            instruction.execute(&mut x, &mut clock, &mut signal_strengths);
        }

        Ok(signal_strengths.iter().sum())
    }
}

fn parse_line(line: &str) -> Result<Instruction, ParseError> {
    let mut splits = line.split_whitespace();

    match (splits.next(), splits.next()) {
        (Some("addx"), Some(value)) => Ok(Instruction::Addx(value.parse()?)),
        (Some("noop"), _) => Ok(Instruction::Noop),
        _ => Err(ParseError::Invalid),
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    use super::Instruction;

    fn get_input() -> Vec<Instruction> {
        // {{{
        vec![
            Instruction::Addx(15),
            Instruction::Addx(-11),
            Instruction::Addx(6),
            Instruction::Addx(-3),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(-8),
            Instruction::Addx(13),
            Instruction::Addx(4),
            Instruction::Noop,
            Instruction::Addx(-1),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(5),
            Instruction::Addx(-1),
            Instruction::Addx(-35),
            Instruction::Addx(1),
            Instruction::Addx(24),
            Instruction::Addx(-19),
            Instruction::Addx(1),
            Instruction::Addx(16),
            Instruction::Addx(-11),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(21),
            Instruction::Addx(-15),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(-3),
            Instruction::Addx(9),
            Instruction::Addx(1),
            Instruction::Addx(-3),
            Instruction::Addx(8),
            Instruction::Addx(1),
            Instruction::Addx(5),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(-36),
            Instruction::Noop,
            Instruction::Addx(1),
            Instruction::Addx(7),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(2),
            Instruction::Addx(6),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(7),
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Addx(-13),
            Instruction::Addx(13),
            Instruction::Addx(7),
            Instruction::Noop,
            Instruction::Addx(1),
            Instruction::Addx(-33),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(2),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(8),
            Instruction::Noop,
            Instruction::Addx(-1),
            Instruction::Addx(2),
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Addx(17),
            Instruction::Addx(-9),
            Instruction::Addx(1),
            Instruction::Addx(1),
            Instruction::Addx(-3),
            Instruction::Addx(11),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(-13),
            Instruction::Addx(-19),
            Instruction::Addx(1),
            Instruction::Addx(3),
            Instruction::Addx(26),
            Instruction::Addx(-30),
            Instruction::Addx(12),
            Instruction::Addx(-1),
            Instruction::Addx(3),
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(-9),
            Instruction::Addx(18),
            Instruction::Addx(1),
            Instruction::Addx(2),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(9),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(-1),
            Instruction::Addx(2),
            Instruction::Addx(-37),
            Instruction::Addx(1),
            Instruction::Addx(3),
            Instruction::Noop,
            Instruction::Addx(15),
            Instruction::Addx(-21),
            Instruction::Addx(22),
            Instruction::Addx(-6),
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Addx(2),
            Instruction::Addx(1),
            Instruction::Noop,
            Instruction::Addx(-10),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Addx(20),
            Instruction::Addx(1),
            Instruction::Addx(2),
            Instruction::Addx(2),
            Instruction::Addx(-6),
            Instruction::Addx(-11),
            Instruction::Noop,
            Instruction::Noop,
            Instruction::Noop,
        ]
        //}}}
    }

    #[test]
    fn parsing() {
        //{{{
        let input = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        //}}}

        let input = super::Solver::parse(String::from(input));
        assert_eq!(input.unwrap(), get_input());
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(super::Solver::part_1(input).unwrap(), 13140);
    }
}
