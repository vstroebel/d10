use std::io::Error as IoError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DecodingError {
    BadFileExtension(String),
    UnknownFormat,
    InvalidBufferSize { width: u32, height: u32 },
    Decoding(String),
    IoError(IoError),
}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DecodingError::*;
        match self {
            BadFileExtension(path) => write!(f, "Bad file extension: {}", path),
            UnknownFormat => write!(f, "Unknown format"),
            InvalidBufferSize { width, height } => write!(f, "Unsupported buffer size for image: {}x{}", width, height),
            Decoding(message) => write!(f, "{}", message),
            IoError(err) => err.fmt(f),
        }
    }
}

impl Error for DecodingError {}

impl From<IoError> for DecodingError {
    fn from(err: IoError) -> DecodingError {
        DecodingError::IoError(err)
    }
}

#[derive(Debug)]
pub enum EncodingError {
    BadFileExtension(String),
    BadDimensions { format: &'static str, width: u32, height: u32 },
    Encoding(String),
    IoError(IoError),
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use EncodingError::*;
        match self {
            BadFileExtension(path) => write!(f, "Bad file extension: {}", path),
            BadDimensions { format, width, height } => write!(f, "Image dimensions not supported by format {}: {}x{}", format, width, height),
            Encoding(message) => write!(f, "{}", message),
            IoError(err) => err.fmt(f),
        }
    }
}

impl Error for EncodingError {}

impl From<IoError> for EncodingError {
    fn from(err: IoError) -> EncodingError {
        EncodingError::IoError(err)
    }
}
