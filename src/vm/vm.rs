use crate::chunk::*;
use crate::utils::print::{print_value, print_stack};

#[derive(Debug)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct Vm<'a> {
    chunk: Option<&'a mut Chunk<'a>>,
    ip: Option<&'a OpCode<'a>>,
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

    pub fn interpret(&mut self, chunk: &'a mut Chunk<'a>) -> InterpretResult {
        self.chunk = Some(chunk);
        // test hipotesis for use immutable refences for opcode so
        // it doesn't needs to be mutable, it will copy any value
        // inside it if needed to perform the instruction actions
        // (supposelly) freeing memory after that.
        // points to first element of code stack
        self.ip = Some(self.chunk
            .as_ref().unwrap()
            .code.first()
            .unwrap()
        );

        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        let mut operation_status = InterpretResult::CompileError;
        let chunk = self.chunk.as_mut().unwrap();

        for opcode in chunk.code.iter() {
            operation_status = match opcode {
                OpCode::OpReturn => {
                   print_value(chunk.pop());

                   return InterpretResult::Ok
                },
                OpCode::OpConstant(index) => {
                    let constant = chunk.constants[**index];
                    chunk.push(constant);

                    return InterpretResult::Ok
                },
                _ => InterpretResult::RuntimeError 
            }
        }

        operation_status
    }
}