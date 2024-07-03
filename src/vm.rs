use crate::chunk::*;
use crate::compiler::compile;
use crate::utils::print::print_value;
use crate::value::Value;

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct Vm {
    chunk: Option<Chunk>,
}

impl Default for Vm {
    fn default() -> Self {
        Self {
            chunk: Some(Chunk::default()),
        }
    }
}

impl Vm {
    pub fn interpret(&mut self, source: &[char]) -> InterpretResult {
        let (chunk, result) = compile(source);

        if result != InterpretResult::Ok {
            panic!("{:?}", result);
        }

        self.chunk = Some(chunk);
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        let mut op_status = InterpretResult::CompileError;

        for i in 0..self.chunk.as_ref().unwrap().code.len() {
            let opcode = &self.chunk.as_ref().unwrap().code[i];
            // print_stack(&self.chunk.as_ref().unwrap());
            op_status = match opcode {
                OpCode::Return => {
                    {
                        let chunk = self.chunk.as_mut().unwrap();
                        print_value(&chunk.stack.pop().expect("stack underflow."));
                    }

                    InterpretResult::Ok
                }
                OpCode::Constant(index) => {
                    let temp_index = *index;
                    {
                        let chunk = self.chunk.as_mut().unwrap();
                        let constant = chunk.constants[temp_index];
                        chunk.stack.push(constant);
                    }

                    InterpretResult::Ok
                }
                OpCode::Negate => {
                    {
                        let chunk = self.chunk.as_mut().unwrap();
                        let to_be_negated = chunk.stack.pop().unwrap();

                        match to_be_negated {
                            Value::Int(value) => chunk.stack.push(Value::Int(-value)),
                            Value::Float(value) => chunk.stack.push(Value::Float(-value)),
                        }
                    }

                    InterpretResult::Ok
                }
                OpCode::Add => {
                    self.binary_op("+");

                    InterpretResult::Ok
                }
                OpCode::Multiply => {
                    self.binary_op("*");

                    InterpretResult::Ok
                }
                OpCode::Divide => {
                    self.binary_op("/");

                    InterpretResult::Ok
                }
            };

            dynamize_stack_vec(&mut self.chunk.as_mut().unwrap().stack);
        }

        op_status
    }

    fn binary_op(&mut self, op: &str) -> InterpretResult {
        let b = self
            .chunk
            .as_mut()
            .unwrap()
            .stack
            .pop()
            .expect("value b not loaded.");
        let a = self
            .chunk
            .as_mut()
            .unwrap()
            .stack
            .pop()
            .expect("value a not loaded.");

        match op {
            "+" => self.chunk.as_mut().unwrap().stack.push(a + b),
            "*" => self.chunk.as_mut().unwrap().stack.push(a * b),
            "/" => self.chunk.as_mut().unwrap().stack.push(a / b),
            _ => panic!("invalid operation."),
        }

        InterpretResult::Ok
    }
}
