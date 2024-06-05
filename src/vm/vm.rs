use crate::chunk::*;
use crate::utils::print::print_value;

#[derive(Debug)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct Vm<'a> {
    chunk: Option<&'a Chunk<'a>>,
    ip: Option<&'a Vec<&'a OpCode<'a>>>,
}

// REVIEW set correct VM structure
// static mut VM: Vm = Vm::new();

impl<'a> Vm<'a> {
    pub fn new() -> Self {
        Vm {
            chunk: None,
            ip: None,
        }
    }

    pub fn interpret(&mut self, chunk: &'a Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        self.ip = Some(&chunk.code);
        self.run()
    }

    fn run(&self) -> InterpretResult {
        let mut result = InterpretResult::CompileError;

        match self.ip {
            Some(vec) => {
                for &opcode in vec.iter() {
                    match opcode {
                        OpCode::OpReturn => result = InterpretResult::Ok,
                        OpCode::OpConstant(index) => {
                            let constant = self.chunk.unwrap().constants[**index];
                            print_value(constant);
                            result = InterpretResult::Ok
                        }
                    }
                }
            }
            None => result = InterpretResult::RuntimeError,
        }

        result
    }
}
