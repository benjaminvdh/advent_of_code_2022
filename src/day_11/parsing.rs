use std::sync::mpsc::{channel, Receiver, Sender};

use crate::ParseError;

use super::Monkey;

pub fn parse_monkey(input: &str) -> Result<Monkey, ParseError> {
    let mut lines = input.lines();

    let items_line = lines.nth(1).ok_or(ParseError::Incomplete)?;
    let operation_line = lines.next().ok_or(ParseError::Incomplete)?;
    let test_line = lines.next().ok_or(ParseError::Incomplete)?;
    let true_line = lines.next().ok_or(ParseError::Incomplete)?;
    let false_line = lines.next().ok_or(ParseError::Incomplete)?;

    let (tx, rx) = parse_items(items_line)?;

    Ok(Monkey {
        items_in: tx,
        items_out: rx,
        operation: parse_operation(operation_line)?,
        test: parse_test(test_line)?,
        true_monkey: parse_true_monkey(true_line)?,
        false_monkey: parse_false_monkey(false_line)?,
    })
}

fn parse_items(items: &str) -> Result<(Sender<usize>, Receiver<usize>), ParseError> {
    let (tx, rx) = channel::<usize>();

    let items = items
        .strip_prefix("  Starting items:")
        .ok_or(ParseError::Invalid)?;

    for item in items.split(',') {
        let _ = tx.send(item.trim().parse()?);
    }

    Ok((tx, rx))
}

fn parse_operation(operation: &str) -> Result<Box<dyn Fn(usize) -> usize>, ParseError> {
    let operation = operation
        .strip_prefix("  Operation: new = old ")
        .ok_or(ParseError::Invalid)?;

    let op = operation.chars().next().ok_or(ParseError::Incomplete)?;

    if &operation[2..] == "old" {
        if op == '*' {
            Ok(Box::new(|old| old * old))
        } else if op == '+' {
            Ok(Box::new(|old| old + old))
        } else {
            Err(ParseError::Invalid)
        }
    } else {
        let number: usize = operation[2..].parse()?;

        if op == '*' {
            Ok(Box::new(move |old| old * number))
        } else if op == '+' {
            Ok(Box::new(move |old| old + number))
        } else {
            Err(ParseError::Invalid)
        }
    }
}

fn parse_test(test: &str) -> Result<Box<dyn Fn(usize) -> bool>, ParseError> {
    let test = test
        .strip_prefix("  Test: divisible by ")
        .ok_or(ParseError::Invalid)?;

    let divisor: usize = test.parse()?;
    Ok(Box::new(move |test| test % divisor == 0))
}

fn parse_true_monkey(monkey: &str) -> Result<usize, ParseError> {
    let monkey = monkey
        .strip_prefix("    If true: throw to monkey ")
        .ok_or(ParseError::Invalid)?;

    Ok(monkey.parse()?)
}

fn parse_false_monkey(monkey: &str) -> Result<usize, ParseError> {
    let monkey = monkey
        .strip_prefix("    If false: throw to monkey ")
        .ok_or(ParseError::Invalid)?;

    Ok(monkey.parse()?)
}
