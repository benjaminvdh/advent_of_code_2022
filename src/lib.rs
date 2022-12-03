pub mod day_1;
pub mod day_2;
pub mod day_3;

mod input;
mod macros;
mod parsing;
mod solving;

use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{self, BufReader, Read};

use input::InputError;
use parsing::ParseError;
use solving::{SolveError, Solver};

#[derive(Debug)]
pub enum AocError {
    Input(InputError),
    Io(io::Error),
    Parsing(ParseError),
    Solving(SolveError),
}

impl From<InputError> for AocError {
    fn from(e: InputError) -> Self {
        AocError::Input(e)
    }
}

impl From<io::Error> for AocError {
    fn from(e: io::Error) -> Self {
        AocError::Io(e)
    }
}

impl From<ParseError> for AocError {
    fn from(e: ParseError) -> Self {
        AocError::Parsing(e)
    }
}

impl From<SolveError> for AocError {
    fn from(e: SolveError) -> Self {
        AocError::Solving(e)
    }
}

impl Display for AocError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            AocError::Input(e) => e.fmt(f),
            AocError::Io(e) => e.fmt(f),
            AocError::Parsing(e) => e.fmt(f),
            AocError::Solving(e) => e.fmt(f),
        }
    }
}

pub fn run<S: Solver>() {
    let part_1 = get_input().and_then(|file| solve_part_1::<S, _>(file));
    print_solve_result(S::DAY, 1, part_1);

    let part_2 = get_input().and_then(|file| solve_part_2::<S, _>(file));
    print_solve_result(S::DAY, 2, part_2);
}

fn get_input() -> Result<File, AocError> {
    let file = input::get_input_file()?;

    Ok(file)
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
