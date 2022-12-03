pub mod day_1;
pub mod day_2;
pub mod day_3;

mod input;
mod macros;
mod parsing;
mod solving;

use std::fmt::{self, Display, Formatter};
use std::io::{BufReader, Read};

use input::InputError;
use parsing::ParseError;
use solving::{SolveError, Solver};

pub fn run<S: Solver>() {
    let part_1 = input::get_input_file().and_then(|file| solve_part_1::<S, _>(file));
    print_solve_result(S::DAY, 1, part_1);

    let part_2 = input::get_input_file().and_then(|file| solve_part_2::<S, _>(file));
    print_solve_result(S::DAY, 2, part_2);
}

pub fn solve_part_1<S: Solver, R: Read>(input: R) -> Result<u64, AocError> {
    let input = S::parse(BufReader::new(input))?;
    let result = S::part_1(input)?;

    Ok(result)
}

pub fn solve_part_2<S: Solver, R: Read>(input: R) -> Result<u64, AocError> {
    let input = S::parse(BufReader::new(input))?;
    let result = S::part_2(input)?;

    Ok(result)
}

fn print_solve_result(day: u8, part: u8, result: Result<u64, AocError>) {
    match result {
        Ok(result) => println!("The result of day {} part {} is {}", day, part, result),
        Err(e) => eprintln!("Failed to solve day {} part {}: {}", day, part, e),
    }
}

#[derive(Debug)]
pub enum AocError {
    Input(InputError),
    Parsing(ParseError),
    Solving(SolveError),
}

impl Display for AocError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            AocError::Input(e) => e.fmt(f),
            AocError::Parsing(e) => e.fmt(f),
            AocError::Solving(e) => e.fmt(f),
        }
    }
}
