pub mod day_1;

mod input;
mod parsing;
mod solving;

use std::fmt::{self, Display, Formatter};
use std::io::{self, BufReader, Read};

use input::InputError;
use parsing::ParseError;
use solving::{Solver, SolveResult};

#[derive(Debug)]
pub enum AocError {
    Input(InputError),
    Io(io::Error),
    Parsing(ParseError),
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

impl Display for AocError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            AocError::Input(e) => e.fmt(f),
            AocError::Io(e) => e.fmt(f),
            AocError::Parsing(e) => e.fmt(f),
        }
    }
}

pub fn run<S: Solver>() {
    match solve::<S>() {
        Ok((part_1, part_2)) => {
            print_solve_result(S::DAY, 1, &part_1);
            print_solve_result(S::DAY, 2, &part_2);
        },
        Err(e) => eprintln!("{}", e),
    }
}

fn solve<S: Solver>() -> Result<(SolveResult, SolveResult), AocError> {
    let input = input::get_input()?;

    solve_on_input::<S, _>(input)
}

pub fn solve_on_input<S: Solver, R: Read>(input: BufReader<R>) -> Result<(SolveResult, SolveResult), AocError> {
    let input = S::parse(input)?;

    Ok((S::part_1(&input), S::part_2(&input)))
}

fn print_solve_result(day: u8, part: u8, result: &SolveResult) {
    match result {
        Ok(result) => println!("The result of day {} part {} is {}.", day, part, result),
        Err(e) => eprintln!("{}", e),
    }
}
