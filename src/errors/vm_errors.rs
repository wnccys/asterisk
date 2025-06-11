use std::{error, fmt};

pub type VmResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum InterpretResult {
    Ok,
    RuntimeError,
    CompilerError,
}

impl fmt::Display for InterpretResult {
    fn fmt() {}
}
