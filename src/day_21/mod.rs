use std::collections::HashMap;

use crate::{ParseError, SolveError};

#[derive(Debug, PartialEq)]
pub enum Monkey {
    Operand(i64),
    Operation(Operation),
}

impl Monkey {
    pub fn get_answer(&self, monkeys: &Monkeys) -> Result<i64, SolveError> {
        match self {
            Monkey::Operand(answer) => Ok(*answer),
            Monkey::Operation(op) => op.execute(&monkeys),
        }
    }
}

pub type Monkeys = HashMap<String, Monkey>;

#[derive(Debug, PartialEq)]
pub struct Operation {
    a: String,
    b: String,
    f: Function,
}

impl Operation {
    fn execute(&self, monkeys: &Monkeys) -> Result<i64, SolveError> {
        let a = monkeys.get(&self.a).ok_or(SolveError::InvalidInput)?;
        let a = a.get_answer(monkeys)?;

        let b = monkeys.get(&self.b).ok_or(SolveError::InvalidInput)?;
        let b = b.get_answer(monkeys)?;

        Ok(self.f.execute(a, b))
    }
}

#[derive(Debug, PartialEq)]
enum Function {
    Add,
    Sub,
    Mul,
    Div,
}

impl Function {
    fn execute(&self, a: i64, b: i64) -> i64 {
        match self {
            Function::Add => a + b,
            Function::Sub => a - b,
            Function::Mul => a * b,
            Function::Div => a / b,
        }
    }
}

pub struct Solver {}

impl crate::Solver for Solver {
    type Input = Monkeys;
    type Output = i64;
    const DAY: u8 = 21;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        Ok(input
            .lines()
            .map(|line| parse_line(line))
            .collect::<Result<_, _>>()?)
    }

    fn part_1(monkeys: Self::Input) -> Result<Self::Output, SolveError> {
        monkeys
            .get("root")
            .ok_or(SolveError::InvalidInput)
            .and_then(|root| root.get_answer(&monkeys))
    }
}

fn parse_line(line: &str) -> Result<(String, Monkey), ParseError> {
    let (name, line) = line.split_once(':').ok_or(ParseError::Invalid)?;

    let name = name.to_owned();
    let line = line.trim_start();

    if let Ok(operand) = line.parse::<i64>() {
        Ok((name, Monkey::Operand(operand)))
    } else {
        let mut splits = line.splitn(3, ' ');
        let a = splits.next().ok_or(ParseError::Invalid)?.to_owned();
        let op = splits.next().ok_or(ParseError::Invalid)?;
        let b = splits.next().ok_or(ParseError::Invalid)?.to_owned();

        let f = match op {
            "+" => Ok(Function::Add),
            "-" => Ok(Function::Sub),
            "*" => Ok(Function::Mul),
            "/" => Ok(Function::Div),
            _ => Err(ParseError::Invalid),
        }?;

        let operation = Operation { a, b, f };
        Ok((name, Monkey::Operation(operation)))
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    use super::*;

    fn number(name: &str, value: i64) -> (String, Monkey) {
        (name.to_owned(), Monkey::Operand(value))
    }

    fn operation(name: &str, a: &str, b: &str, f: Function) -> (String, Monkey) {
        let operation = Operation {
            a: String::from(a),
            b: String::from(b),
            f,
        };

        (name.to_owned(), Monkey::Operation(operation))
    }

    fn get_input() -> Monkeys {
        vec![
            operation("root", "pppw", "sjmn", Function::Add),
            number("dbpl", 5),
            operation("cczh", "sllz", "lgvd", Function::Add),
            number("zczc", 2),
            operation("ptdq", "humn", "dvpt", Function::Sub),
            number("dvpt", 3),
            number("lfqf", 4),
            number("humn", 5),
            number("ljgn", 2),
            operation("sjmn", "drzm", "dbpl", Function::Mul),
            number("sllz", 4),
            operation("pppw", "cczh", "lfqf", Function::Div),
            operation("lgvd", "ljgn", "ptdq", Function::Mul),
            operation("drzm", "hmdt", "zczc", Function::Sub),
            number("hmdt", 32),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn parsing() {
        let input = r"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

        assert_eq!(
            super::Solver::parse(String::from(input)).unwrap(),
            get_input()
        );
    }

    #[test]
    fn part_1() {
        let input = get_input();

        assert_eq!(super::Solver::part_1(input).unwrap(), 152);
    }
}
