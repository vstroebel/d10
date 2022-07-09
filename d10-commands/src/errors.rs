use d10::{DecodingError, EncodingError};
use std::error::Error;
use std::fmt::{Display, Formatter};

pub type CommandResult<T> = Result<T, CommandError>;

#[derive(Debug)]
pub enum CommandError {
    MissingImage,
    Decoding(DecodingError),
    Encoding(EncodingError),
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::MissingImage => write!(f, "Missing image"),
            CommandError::Decoding(err) => err.fmt(f),
            CommandError::Encoding(err) => err.fmt(f),
        }
    }
}

impl Error for CommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CommandError::Decoding(err) => Some(err),
            CommandError::Encoding(err) => Some(err),
            _ => None,
        }
    }
}

impl From<DecodingError> for CommandError {
    fn from(err: DecodingError) -> Self {
        CommandError::Decoding(err)
    }
}

impl From<EncodingError> for CommandError {
    fn from(err: EncodingError) -> Self {
        CommandError::Encoding(err)
    }
}
