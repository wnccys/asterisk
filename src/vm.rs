use crate::chunk::*;
use crate::compiler::compile;
use crate::types::Table;
use crate::utils::print::{print_stack, print_value};
use crate::value::{values_equal, Value};

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct Vm {
    chunk: Box<Chunk>,
    globals: Table,
    strings: Table,
}

impl Default for Vm {
    fn default() -> Self {
        Self {
            chunk: Box::default(),
            globals: Table::default(),
            strings: Table::default(),
        }
    }
}

impl Vm {
    pub fn interpret(&mut self, source: Vec<char>) -> InterpretResult {
        let (chunk, result) = compile(&mut self.strings, source);

        if result != InterpretResult::Ok {
            panic!("{:?}", result);
        }

        self.chunk = Box::new(chunk);
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        let mut op_status = InterpretResult::CompileError;

        for i in 0..self.chunk.as_ref().code.len() {
            let opcode = &self.chunk.as_ref().code[i];

            print_stack(self.chunk.as_ref());

            op_status = match opcode {
                OpCode::Return => {
                    {
                        let chunk = self.chunk.as_mut();
                        print_value(
                            &chunk
                                .stack
                                .pop()
                                .expect("Error on return: stack underflow."),
                        );
                    }

                    InterpretResult::Ok
                }
                OpCode::Constant(index) => {
                    let temp_index = *index;
                    {
                        let chunk = self.chunk.as_mut();
                        let constant = chunk.constants[temp_index].clone();
                        chunk.stack.push(constant);
                    }

                    InterpretResult::Ok
                }
                OpCode::Negate => {
                    {
                        let chunk = self.chunk.as_mut();
                        let to_be_negated = chunk.stack.pop().unwrap();

                        match to_be_negated {
                            Value::Int(value) => chunk.stack.push(Value::Int(-value)),
                            Value::Float(value) => chunk.stack.push(Value::Float(-value)),
                            Value::Bool(value) => chunk.stack.push(Value::Bool(!value)),
                            _ => todo!("Operation not allowed."),
                        }
                    }

                    InterpretResult::Ok
                }
                OpCode::Not => {
                    let chunk = self.chunk.as_mut();
                    let to_be_negated = chunk.stack.pop().unwrap();

                    match to_be_negated {
                        Value::Bool(value) => chunk.stack.push(Value::Bool(!value)),
                        _ => panic!("Value should be a boolean."),
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
                OpCode::True => {
                    let chunk = self.chunk.as_mut();
                    chunk.stack.push(Value::Bool(true));

                    InterpretResult::Ok
                }
                OpCode::False => {
                    let chunk = self.chunk.as_mut();
                    chunk.stack.push(Value::Bool(false));

                    InterpretResult::Ok
                }
                OpCode::Equal => {
                    let chunk = self.chunk.as_mut();
                    let a = chunk.stack.pop().unwrap();
                    let b = chunk.stack.pop().unwrap();

                    chunk.stack.push(values_equal(a, b));

                    InterpretResult::Ok
                }
                OpCode::Greater => {
                    self.binary_op(">");

                    InterpretResult::Ok
                }
                OpCode::Less => {
                    self.binary_op("<");

                    InterpretResult::Ok
                }
                OpCode::Print => {
                    let chunk = &self.chunk.as_mut().stack.pop().unwrap();
                    print_value(chunk);

                    InterpretResult::Ok
                }
                OpCode::Pop => {
                    let chunk = self.chunk.as_mut();

                    chunk.stack.pop().expect("Error on pop: stack underflow.");

                    InterpretResult::Ok
                }
                OpCode::Nil => InterpretResult::Ok,
                OpCode::DefineGlobal(var_index) => {
                    let temp_index = *var_index;

                    let chunk = self.chunk.as_mut();
                    let var_name = chunk.constants[temp_index].clone();

                    match var_name {
                        Value::String(name) => {
                            self.globals.set(&name, chunk.stack.pop().unwrap());
                        }
                        _ => panic!("Invalid global variable name."),
                    }

                    InterpretResult::Ok
                }
                // TODO implement better global var get
                OpCode::GetGlobal(var_index) => {
                    let temp_index = *var_index;
                    let chunk = self.chunk.as_mut();

                    let name = match &chunk.constants[temp_index] {
                        Value::String(name) => name,
                        _ => panic!("Invalid global variable name."),
                    };

                    let value = match self.globals.get(name) {
                        Some(value) => value.value.clone(),
                        _ => panic!(
                            "Use of undeclared variable '{}'",
                            name.into_iter().collect::<String>()
                        ),
                    };

                    chunk.stack.push(value);

                    InterpretResult::Ok
                }
                OpCode::SetGlobal(index) => {
                    let temp_index = *index;
                    let chunk = self.chunk.as_mut();

                    let name = match &chunk.constants[temp_index] {
                        Value::String(name) => name,
                        _ => panic!("Invalid global variable name."),
                    };

                    dbg!(self
                        .globals
                        .set(name, chunk.stack.iter().last().unwrap().clone().to_owned()));

                    dbg!(chunk.stack.iter().last().unwrap().clone().to_owned());

                    if self
                        .globals
                        .set(name, chunk.stack.iter().last().unwrap().clone().to_owned())
                    {
                        let _ = self.globals.delete(name);
                        panic!("Global variable is used before it's initialization.");
                    }
                    break;

                    // InterpretResult::Ok
                }
            };
        }

        op_status
    }

    fn binary_op(&mut self, op: &str) -> InterpretResult {
        let b = self
            .chunk
            .as_mut()
            .stack
            .pop()
            .expect("value b not loaded.");

        let a = self
            .chunk
            .as_mut()
            .stack
            .pop()
            .expect("value a not loaded.");

        match op {
            "+" => self.chunk.as_mut().stack.push(a + b),
            "*" => self.chunk.as_mut().stack.push(a * b),
            "/" => self.chunk.as_mut().stack.push(a / b),
            // REVIEW check for >, < partialOrd inconvenient (apply the condition on other variants)
            // in this case a same type as b is false;
            ">" => self.chunk.as_mut().stack.push(Value::Bool(a > b)),
            "<" => self.chunk.as_mut().stack.push(Value::Bool(a < b)),
            _ => panic!("invalid operation."),
        }

        InterpretResult::Ok
    }
}
