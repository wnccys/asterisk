use std::{error, fmt};

pub type VmResult = Result<(), VmError>;

#[derive(Debug)]
pub struct VmError {
    pub message: String,
    pub _type: InterpretResult,
}

impl VmError {
    pub fn new(message: String, _type: InterpretResult) -> Self {
        VmError { message, _type }
    }
}

#[derive(Debug)]
pub enum InterpretResult {
    RuntimeError,
    CompilerError,
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self._type {
            InterpretResult::RuntimeError => write!(f, "Runtime error: {}", self.message),
            InterpretResult::CompilerError => {
                write!(f, "An compilation error occurred: {}", self.message)
            }
        }
    }
}

impl error::Error for VmError {}
