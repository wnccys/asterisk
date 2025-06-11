use std::{error, fmt};

pub type ParserResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct xError {
    message: &'static str
}

impl fmt::Display for xError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error")
    }
}

impl error::Error for xError {}
