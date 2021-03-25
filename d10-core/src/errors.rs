use std::io::Error as IOError;
use std::error::Error;
use std::fmt;

pub type D10Result<T> = Result<T, D10Error>;

#[derive(Debug)]
pub enum D10Error {
    OpenError(String),
    SaveError(String),
    MissingImage,
    Limits(String),
    IOError(IOError),
    BadArgument(String),
}

impl fmt::Display for D10Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for D10Error {}

impl From<IOError> for D10Error {
    fn from(err: IOError) -> D10Error {
        D10Error::IOError(err)
    }
}

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
