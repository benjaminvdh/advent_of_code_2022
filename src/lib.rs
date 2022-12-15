pub mod day_1;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;

mod input;
mod macros;
mod parsing;
mod solving;

use std::fmt::{self, Display, Formatter};
use std::time::{Duration, Instant};

use input::InputError;
use parsing::ParseError;
use solving::{SolveError, Solver};

pub fn run<S: Solver>() {
    let start = Instant::now();
    let part_1 = input::get_input().and_then(|input| solve_part_1::<S>(input));
    let elapsed = start.elapsed();
    print_solve_result(S::DAY, 1, part_1, elapsed);

    let start = Instant::now();
    let part_2 = input::get_input().and_then(|input| solve_part_2::<S>(input));
    let elapsed = start.elapsed();
    print_solve_result(S::DAY, 2, part_2, elapsed);
}

pub fn solve_part_1<S: Solver>(input: String) -> Result<S::Output, AocError> {
    let input = S::parse(input)?;
    let result = S::part_1(input)?;

    Ok(result)
}

pub fn solve_part_2<S: Solver>(input: String) -> Result<S::Output, AocError> {
    let input = S::parse(input)?;
    let result = S::part_2(input)?;

    Ok(result)
}

fn print_solve_result<D: Display>(
    day: u8,
    part: u8,
    result: Result<D, AocError>,
    elapsed: Duration,
) {
    match result {
        Ok(result) => println!(
            "The result of day {} part {} is {} (solved in {})",
            day,
            part,
            result,
            duration_to_string(elapsed)
        ),
        Err(e) => eprintln!("Failed to solve day {} part {}: {}", day, part, e),
    }
}

fn duration_to_string(duration: Duration) -> String {
    format!(
        "{}.{:0>3} {:0>3} {:0>3} s",
        duration.as_secs(),
        duration.as_millis(),
        duration.as_micros() % 1_000,
        duration.as_nanos() % 1_000,
    )
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
