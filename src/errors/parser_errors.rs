use std::{error, fmt};

pub type ParserResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct xError;

impl fmt::Display for xError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error")
    }
}
