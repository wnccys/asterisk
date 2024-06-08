use crate::chunk::*;
use crate::utils::print::{print_value, print_stack};
use crate::value::Value;
use std::ops;

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

    fn run (&mut self) -> InterpretResult {
        let mut op_status = InterpretResult::CompileError;

        for i in 0..self.chunk.as_ref().unwrap().code.len() {
            let opcode = &self.chunk.as_ref().unwrap().code[i];

            op_status = match opcode {
                OpCode::OpReturn => {
                    {
                        let chunk = self.chunk.as_mut().unwrap();
                        chunk.pop_stack();
                    }

                    InterpretResult::Ok
                },
                OpCode::OpConstant(index) => {
                    {
                        let chunk = self.chunk.as_mut().unwrap();
                        let constant = chunk.constants[**index];
                        chunk.push_stack(constant);
                        print_value(&constant);
                    }

                    InterpretResult::Ok
                },
                OpCode::OpNegate => {
                    let value = {
                        let chunk = self.chunk.as_mut().unwrap();
                        chunk.pop_stack().negate()
                    };

                    let chunk = self.chunk.as_mut().unwrap();
                    chunk.push_stack(value);
                    print_value(&value);
                     
                    InterpretResult::Ok
                },
                _ => InterpretResult::RuntimeError 
            }
        }

        op_status
    }

    fn binary_op(&mut self, op: fn(f32,f32) -> f32) {
    }
}