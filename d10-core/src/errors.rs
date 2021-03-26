use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseEnumError {
    pub input: String,
    pub enum_type: &'static str,
}

impl ParseEnumError {
    pub fn new(input: &str, enum_name: &'static str) -> ParseEnumError {
        ParseEnumError {
            input: input.to_owned(),
            enum_type: enum_name,
        }
    }
}

impl fmt::Display for ParseEnumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown value for {}: {}", self.enum_type, self.input)
    }
}

impl Error for ParseEnumError {}
