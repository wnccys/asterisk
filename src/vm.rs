use crate::chunk::*;
use crate::utils::print::{print_value, print_stack};
use crate::scanner::{ Scanner, TokenCode };

#[derive(Debug, PartialEq)]
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

    // pub fn interpret(&mut self, chunk: &'a mut Chunk<'a>) -> InterpretResult {
    //     self.chunk = Some(chunk);

    //     self.ip = Some(self.chunk
    //                 .as_ref().unwrap()
    //                 .code.first()
    //                 .unwrap()
    //     );

    //     self.run()
    // }

    pub fn interpret(&mut self, source: &String) -> InterpretResult {
        self.compile(source)
    }

    fn compile(&mut self, source: &String) -> InterpretResult {
        let mut scanner = Scanner::new();
        let mut line = -1;

        loop {
            let token = scanner.scan_token(source);

            if token.line != line {
                print!("{}", token.line);
                line = token.line;
            } else {
                print!("  | ");
            }
            println!("{:?}, {}, {}", token.code , token.length, token.start);

            if token.code == TokenCode::Eof { break };
        }

        InterpretResult::Ok
    }

    fn run (&mut self) -> InterpretResult {
        let mut op_status = InterpretResult::CompileError;

        for i in 0..self.chunk.as_ref().unwrap().code.len() {
            let opcode = &self.chunk.as_ref().unwrap().code[i];
            // print_stack(&self.chunk.as_ref().unwrap());
            op_status = match opcode {
                OpCode::OpReturn => {
                    {
                        let chunk = self.chunk.as_mut().unwrap();
                        print_value(&chunk.stack.pop().expect("stack underflow"));
                    }

                    InterpretResult::Ok
                },
                OpCode::OpConstant(index) => {
                    {
                        let chunk = self.chunk.as_mut().unwrap();
                        let constant = chunk.constants[**index];
                        chunk.stack.push(constant);
                    }

                    InterpretResult::Ok
                },
                OpCode::OpAdd => {
                    self.binary_op("+");

                    InterpretResult::Ok
                },
                OpCode::OpMultiply => {
                    self.binary_op("*");

                    InterpretResult::Ok
                },
                OpCode::OpDivide => {
                    self.binary_op("/");

                    InterpretResult::Ok
                },
                _ => InterpretResult::RuntimeError 
            };

            dynamize_stack_vec(&mut self.chunk.as_mut().unwrap().stack);
        }

        op_status
    }

    fn binary_op(&mut self, op: &str) -> InterpretResult {
        let b = self.chunk.as_mut().unwrap().stack.pop().expect("value b not loaded");
        let a = self.chunk.as_mut().unwrap().stack.pop().expect("value a not loaded");

        match op {
            "+" => self.chunk.as_mut().unwrap().stack.push(a+b),
            "*" => self.chunk.as_mut().unwrap().stack.push(a*b),
            "/" => self.chunk.as_mut().unwrap().stack.push(a/b),
            _ => panic!("invalid operation"),
        }

        InterpretResult::Ok
    }
}