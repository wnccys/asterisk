use std::{error, fmt};

pub type VmResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct InterpretError {
    message: &'static str,
    _type: InterpretResult
}

#[derive(Debug)]
pub enum InterpretResult {
    RuntimeError,
    CompilerError,
}

impl fmt::Display for InterpretError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self._type {
            InterpretResult::RuntimeError => write!(f, "Runtime error: {}", self.message),
            InterpretResult::CompilerError => write!(f, "An compilation error occurred: {}", self.message),
        }
    }
}

impl error::Error for InterpretError {}

