use std::fmt::{self, Display, Formatter};

pub struct ParseError {
    day: u8
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "Failed to parse the input for day {}", self.day)
    }
}
