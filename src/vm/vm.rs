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

impl<'a> Vm<'a> {
    pub fn new() -> Self {
        Vm {
            chunk: None,
            ip: None,
        }
    }

    pub fn interpret(&'a mut self, chunk: &'a mut Chunk<'a>) -> InterpretResult {
        self.chunk = Some(chunk);

        self.ip = Some(self.chunk
                    .as_ref().unwrap()
                    .code.first()
                    .unwrap()
        );

        self.run()
    }

    fn run (&'a mut self) -> InterpretResult {
        let mut operation_status = InterpretResult::CompileError;
        let chunk = self.chunk.as_mut().unwrap();

        for opcode in chunk.code.iter() {
            operation_status = match opcode {
                OpCode::OpReturn => {
                   print_value(&chunk.pop_stack());

                   return InterpretResult::Ok;
                },
                OpCode::OpConstant(index) => {
                    let constant = chunk.constants[**index];
                    chunk.push_stack(constant);

                    return InterpretResult::Ok;
                },
                OpCode::OpNegate => {
                    let value = chunk.pop_stack().negate();
                    chunk.push_stack(value);
                     
                    return InterpretResult::Ok;
                },
                _ => InterpretResult::RuntimeError 
            }
        }

        operation_status
    }
}